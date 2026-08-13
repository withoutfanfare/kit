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
///
/// `Err` means the question could not be answered — no home directory, or a
/// settings file that exists but cannot be read or parsed. A guard that cannot
/// see the hooks must not wave the removal through, so callers refuse instead.
/// A settings file that is simply *absent* is a clear answer: no hooks, `Ok(None)`.
pub fn hook_reference(link_path: &Path) -> Result<Option<String>, String> {
    let Some(home) = dirs::home_dir() else {
        return Err("Cannot locate your home directory, so Kit cannot check \
                    whether a hook runs from this skill."
            .to_string());
    };
    hook_reference_from(&home, link_path)
}

/// The reading half of [`hook_reference`], with the home directory injected so
/// the failure cases can be tested without touching the real config.
pub fn hook_reference_from(home: &Path, link_path: &Path) -> Result<Option<String>, String> {
    let settings = home.join(".claude").join("settings.json");
    let content = match fs::read_to_string(&settings) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => {
            return Err(format!(
                "Cannot read {} ({}), so Kit cannot check whether a hook runs \
                 from this skill.",
                settings.display(),
                e
            ))
        }
    };
    let value: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Cannot parse {} ({}), so Kit cannot check whether a hook runs from \
             this skill. Fix the file and try again.",
            settings.display(),
            e
        )
    })?;
    Ok(hook_reference_in(&value, link_path, home))
}

/// The matching half of [`hook_reference`], with the settings document and home
/// directory injected so it can be tested without touching the real config.
pub fn hook_reference_in(
    settings: &serde_json::Value,
    link_path: &Path,
    home: &Path,
) -> Option<String> {
    // The same folder gets written several ways. `~` is the common one, but
    // `$HOME` and `${HOME}` are just as valid and would otherwise slip past.
    let mut forms = vec![link_path.to_string_lossy().to_string()];
    if let Ok(rest) = link_path.strip_prefix(home) {
        let rest = rest.display().to_string();
        forms.push(format!("~/{rest}"));
        forms.push(format!("$HOME/{rest}"));
        forms.push(format!("${{HOME}}/{rest}"));
    }

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

    found
        .into_iter()
        .find(|cmd| forms.iter().any(|form| references(cmd, form)))
}

/// Remove a symlink at `link_path`. Verifies it is indeed a symlink before
/// removing to avoid accidental deletion of real directories, and refuses when a
/// configured hook runs a script from inside it.
pub fn remove_skill_link(link_path: &Path) -> Result<(), String> {
    // An unanswerable question is a refusal, not a green light: the whole point
    // of the guard is that the damage only shows up in the *next* session.
    if let Some(cmd) = hook_reference(link_path).map_err(|e| {
        format!(
            "Refusing to unlink {}: {}",
            link_path.display(),
            e
        )
    })? {
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
            if let Some(cmd) = hook_reference(&entry.path()).expect("settings readable") {
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

    /// `$HOME` and `${HOME}` mean exactly what `~` means, and a guard that only
    /// knows one of the three spellings is a guard with holes in it.
    #[test]
    fn matches_hook_commands_written_with_home_variables() {
        let home = Path::new("/Users/someone");
        let hooked = home.join(".claude/skills/tracker");

        for command in [
            "bash $HOME/.claude/skills/tracker/log.sh",
            "bash ${HOME}/.claude/skills/tracker/log.sh",
        ] {
            let settings: serde_json::Value = serde_json::from_str(&format!(
                r#"{{"hooks":{{"Stop":[{{"hooks":[{{"type":"command","command":"{command}"}}]}}]}}}}"#
            ))
            .unwrap();
            assert!(
                hook_reference_in(&settings, &hooked, home).is_some(),
                "missed: {command}"
            );
        }
    }

    /// Settings that exist but cannot be parsed leave the guard blind. Reading
    /// that as "no hooks" is the dangerous half of the guess, so the answer is
    /// an error the caller refuses on — not a quiet `None`.
    #[test]
    fn unparseable_settings_are_an_error_not_a_clean_bill_of_health() {
        let base = std::env::temp_dir().join(format!("kit-linker-test-h-{}", std::process::id()));
        fs::remove_dir_all(&base).ok();
        let skills = base.join(".claude").join("skills");
        fs::create_dir_all(&skills).unwrap();
        let link = skills.join("ordinary");

        fs::write(base.join(".claude").join("settings.json"), "{ not json").unwrap();
        let blind = hook_reference_from(&base, &link);
        assert!(blind.is_err(), "a guard that cannot read must not say 'safe'");
        assert!(blind.unwrap_err().contains("Cannot parse"));

        // Readable settings with no hooks are a real answer, and stay removable.
        fs::write(base.join(".claude").join("settings.json"), r#"{"hooks":{}}"#).unwrap();
        assert_eq!(hook_reference_from(&base, &link), Ok(None));

        // No settings file at all is also a real answer: there are no hooks.
        fs::remove_file(base.join(".claude").join("settings.json")).unwrap();
        assert_eq!(hook_reference_from(&base, &link), Ok(None));

        fs::remove_dir_all(&base).ok();
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
