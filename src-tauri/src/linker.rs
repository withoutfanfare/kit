use std::fs;
use std::path::Path;

/// Create a symlink from `link_path` pointing to `target_path`.
/// On Unix this creates a symbolic link. `target_path` should be an absolute
/// path to the library skill folder. `link_path` is the destination inside the
/// location's skills directory.
pub fn create_skill_link(target_path: &Path, link_path: &Path) -> Result<(), String> {
    // Ensure the target exists
    if !target_path.exists() {
        return Err(format!(
            "Target path does not exist: {}",
            target_path.display()
        ));
    }

    // Ensure the parent directory of the link exists
    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directory {}: {}",
                parent.display(),
                e
            )
        })?;
    }

    // If something already exists at the link path, refuse
    if link_path.exists() || fs::symlink_metadata(link_path).is_ok() {
        return Err(format!(
            "Path already exists at link location: {}",
            link_path.display()
        ));
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target_path, link_path).map_err(|e| {
            format!(
                "Failed to create symlink {} -> {}: {}",
                link_path.display(),
                target_path.display(),
                e
            )
        })?;
    }

    #[cfg(not(unix))]
    {
        return Err("Symlink creation is only supported on Unix systems".to_string());
    }

    Ok(())
}

/// Report the hook command that invokes something inside `link_path`, if any.
///
/// Hooks in `~/.claude/settings.json` run scripts by path, for example
/// `python3 ~/.claude/skills/clio-hooks/scripts/session_start.py`. Removing that
/// link breaks session start-up, and the breakage surfaces on the *next* session
/// rather than at the moment of removal — so it has to be caught here.
pub fn hook_reference(link_path: &Path) -> Option<String> {
    let home = dirs::home_dir()?;
    let settings = home.join(".claude").join("settings.json");
    let content = fs::read_to_string(&settings).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    hook_reference_in(&value, link_path, &home)
}

/// The matching half of [`hook_reference`], with the settings document and home
/// directory injected so it can be tested without touching the real config.
pub fn hook_reference_in(
    settings: &serde_json::Value,
    link_path: &Path,
    home: &Path,
) -> Option<String> {
    let absolute = link_path.to_string_lossy().to_string();
    // Hook commands usually write the home directory as `~`.
    let tilde = link_path
        .strip_prefix(home)
        .ok()
        .map(|rest| format!("~/{}", rest.display()));

    fn commands(value: &serde_json::Value, out: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, v) in map {
                    if key == "command" {
                        if let Some(s) = v.as_str() {
                            out.push(s.to_string());
                        }
                    }
                    commands(v, out);
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| commands(v, out)),
            _ => {}
        }
    }

    /// Substring matching alone would flag `clio` because `clio-hooks` starts
    /// with it. A reference only counts when the name ends at a path boundary.
    fn references(cmd: &str, needle: &str) -> bool {
        let mut from = 0;
        while let Some(offset) = cmd[from..].find(needle) {
            let end = from + offset + needle.len();
            let rest = &cmd[end..];
            if rest.is_empty()
                || rest.starts_with('/')
                || rest.starts_with(['"', '\'', ';', ')'])
                || rest.starts_with(char::is_whitespace)
            {
                return true;
            }
            from = end;
        }
        false
    }

    let mut found = Vec::new();
    commands(settings.get("hooks")?, &mut found);

    found.into_iter().find(|cmd| {
        references(cmd, &absolute) || tilde.as_ref().is_some_and(|t| references(cmd, t))
    })
}

/// Remove a symlink at `link_path`. Verifies it is indeed a symlink before
/// removing to avoid accidental deletion of real directories, and refuses when a
/// configured hook runs a script from inside it.
pub fn remove_skill_link(link_path: &Path) -> Result<(), String> {
    if let Some(cmd) = hook_reference(link_path) {
        return Err(format!(
            "Refusing to unlink {}: a hook runs a script from inside it, and removing \
             it would break session start-up.\n\nHook command: {}",
            link_path.display(),
            cmd
        ));
    }

    let meta = fs::symlink_metadata(link_path).map_err(|e| {
        format!(
            "Cannot read metadata for {}: {}",
            link_path.display(),
            e
        )
    })?;

    if !meta.file_type().is_symlink() {
        return Err(format!(
            "Path is not a symlink, refusing to delete: {}",
            link_path.display()
        ));
    }

    fs::remove_file(link_path).map_err(|e| {
        format!("Failed to remove symlink {}: {}", link_path.display(), e)
    })?;

    Ok(())
}

/// Ensure the skills directory exists for a location, creating it if needed.
/// Reuses the scanner's discovery rules so new links land in the same
/// directory the scanner reads from; only defaults to `.claude/skills/`
/// when no skills directory exists yet.
pub fn ensure_skills_dir(
    location_path: &Path,
    kind: crate::domain::LocationKind,
) -> Result<std::path::PathBuf, String> {
    // Global *is* its skills directory; only a project nests one under `.claude/`.
    if kind == crate::domain::LocationKind::Global {
        fs::create_dir_all(location_path).map_err(|e| {
            format!(
                "Failed to create skills directory {}: {}",
                location_path.display(),
                e
            )
        })?;
        return Ok(location_path.to_path_buf());
    }

    let skills_dir = crate::scanner::find_skills_dir(location_path)
        .unwrap_or_else(|| location_path.join(".claude").join("skills"));

    fs::create_dir_all(&skills_dir).map_err(|e| {
        format!(
            "Failed to create skills directory {}: {}",
            skills_dir.display(),
            e
        )
    })?;

    Ok(skills_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A project with both `.claude/` and a top-level `skills/` dir must get
    /// links in `skills/` — where the scanner reads from — not `.claude/skills/`.
    #[test]
    fn ensure_skills_dir_prefers_existing_plain_skills() {
        let base = std::env::temp_dir().join(format!("kit-linker-test-a-{}", std::process::id()));
        let loc = base.join("proj");
        fs::create_dir_all(loc.join("skills")).unwrap();
        fs::create_dir_all(loc.join(".claude")).unwrap();

        let dir = ensure_skills_dir(&loc, crate::domain::LocationKind::Project).unwrap();
        assert_eq!(dir, loc.join("skills"));

        fs::remove_dir_all(&base).ok();
    }

    /// The fixture is the real shape of Danny's settings: a `SessionStart` hook
    /// that runs a script from inside the `clio-hooks` skill. Unlinking it would
    /// break session start-up, and nothing would say so until the next session.
    #[test]
    fn refuses_to_unlink_a_skill_a_hook_runs_from() {
        let home = Path::new("/Users/someone");
        let settings: serde_json::Value = serde_json::from_str(
            r#"{
              "hooks": {
                "SessionStart": [
                  { "hooks": [
                      { "type": "command",
                        "command": "python3 ~/.claude/skills/clio-hooks/scripts/session_start.py" }
                  ] }
                ]
              }
            }"#,
        )
        .unwrap();

        let hooked = home.join(".claude/skills/clio-hooks");
        let found = hook_reference_in(&settings, &hooked, home);
        assert!(found.is_some(), "clio-hooks is referenced and must be caught");
        assert!(found.unwrap().contains("session_start.py"));

        // A skill no hook mentions stays removable.
        let ordinary = home.join(".claude/skills/code-review");
        assert_eq!(hook_reference_in(&settings, &ordinary, home), None);

        // A same-named skill in a *project* is a different path, so it is free to
        // go — the global one is what the hook actually runs.
        let in_project = Path::new("/Users/someone/code/app/.claude/skills/clio-hooks");
        assert_eq!(hook_reference_in(&settings, in_project, home), None);
    }

    /// Environment-dependent smoke test against the machine's real
    /// `~/.claude/settings.json`. Ignored by default because it only means
    /// anything where hook-referenced skills actually exist. Run with:
    /// `cargo test -- --ignored hook_guard_on_this_machine`
    #[test]
    #[ignore]
    fn hook_guard_on_this_machine() {
        let home = dirs::home_dir().expect("home");
        let skills = home.join(".claude").join("skills");
        let Ok(entries) = fs::read_dir(&skills) else {
            eprintln!("no ~/.claude/skills — nothing to check");
            return;
        };

        let mut guarded = Vec::new();
        for entry in entries.flatten() {
            if let Some(cmd) = hook_reference(&entry.path()) {
                guarded.push((entry.file_name().to_string_lossy().to_string(), cmd));
            }
        }

        for (skill, cmd) in &guarded {
            println!("guarded: {skill}\n  via: {cmd}");
        }
        assert!(
            !guarded.is_empty(),
            "expected at least one hook-referenced skill to be protected"
        );
    }

    /// A shorter skill name must not match a longer one that starts with it.
    /// `clio` is a prefix of `clio-hooks`, and blocking its removal would be a
    /// guard that cries wolf — the kind users learn to work around.
    #[test]
    fn does_not_match_a_skill_whose_name_is_merely_a_prefix() {
        let home = Path::new("/Users/someone");
        let settings: serde_json::Value = serde_json::from_str(
            r#"{"hooks":{"SessionStart":[{"hooks":[{"type":"command",
               "command":"python3 ~/.claude/skills/clio-hooks/scripts/session_start.py"}]}]}}"#,
        )
        .unwrap();

        let prefix_named = home.join(".claude/skills/clio");
        assert_eq!(
            hook_reference_in(&settings, &prefix_named, home),
            None,
            "'clio' is not 'clio-hooks' and must stay removable"
        );

        // The real one is still caught.
        let actual = home.join(".claude/skills/clio-hooks");
        assert!(hook_reference_in(&settings, &actual, home).is_some());
    }

    /// Absolute paths in hook commands must be caught too, not just `~` ones.
    #[test]
    fn matches_hook_commands_written_with_an_absolute_path() {
        let home = Path::new("/Users/someone");
        let settings: serde_json::Value = serde_json::from_str(
            r#"{"hooks":{"Stop":[{"hooks":[{"type":"command",
               "command":"/Users/someone/.claude/skills/tracker/log.sh"}]}]}}"#,
        )
        .unwrap();

        let hooked = home.join(".claude/skills/tracker");
        assert!(hook_reference_in(&settings, &hooked, home).is_some());
    }

    /// Global *is* the skills directory. Nesting `.claude/skills` underneath it
    /// would silently write links somewhere Claude Code never reads.
    #[test]
    fn ensure_skills_dir_for_global_is_the_path_itself() {
        let base = std::env::temp_dir().join(format!("kit-linker-test-g-{}", std::process::id()));
        let global = base.join("claude").join("skills");

        let dir = ensure_skills_dir(&global, crate::domain::LocationKind::Global).unwrap();
        assert_eq!(dir, global);
        assert!(dir.is_dir());
        assert!(!global.join(".claude").exists());

        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn ensure_skills_dir_defaults_to_claude_skills() {
        let base = std::env::temp_dir().join(format!("kit-linker-test-b-{}", std::process::id()));
        let loc = base.join("proj");
        fs::create_dir_all(&loc).unwrap();

        let dir = ensure_skills_dir(&loc, crate::domain::LocationKind::Project).unwrap();
        assert_eq!(dir, loc.join(".claude").join("skills"));
        assert!(dir.is_dir());

        fs::remove_dir_all(&base).ok();
    }
}
