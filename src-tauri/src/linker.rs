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

/// Remove a symlink at `link_path`. Verifies it is indeed a symlink before
/// removing to avoid accidental deletion of real directories.
pub fn remove_skill_link(link_path: &Path) -> Result<(), String> {
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
pub fn ensure_skills_dir(location_path: &Path) -> Result<std::path::PathBuf, String> {
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

        let dir = ensure_skills_dir(&loc).unwrap();
        assert_eq!(dir, loc.join("skills"));

        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn ensure_skills_dir_defaults_to_claude_skills() {
        let base = std::env::temp_dir().join(format!("kit-linker-test-b-{}", std::process::id()));
        let loc = base.join("proj");
        fs::create_dir_all(&loc).unwrap();

        let dir = ensure_skills_dir(&loc).unwrap();
        assert_eq!(dir, loc.join(".claude").join("skills"));
        assert!(dir.is_dir());

        fs::remove_dir_all(&base).ok();
    }
}
