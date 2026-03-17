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
/// Returns the path to the skills directory (preferring `.claude/skills/`).
pub fn ensure_skills_dir(location_path: &Path) -> Result<std::path::PathBuf, String> {
    // Prefer .claude/skills/ if .claude/ already exists
    let claude_dir = location_path.join(".claude");
    let skills_dir = if claude_dir.is_dir() {
        claude_dir.join("skills")
    } else {
        // Check if skills/ already exists
        let plain = location_path.join("skills");
        if plain.is_dir() {
            plain
        } else {
            // Default to .claude/skills/
            claude_dir.join("skills")
        }
    };

    fs::create_dir_all(&skills_dir).map_err(|e| {
        format!(
            "Failed to create skills directory {}: {}",
            skills_dir.display(),
            e
        )
    })?;

    Ok(skills_dir)
}
