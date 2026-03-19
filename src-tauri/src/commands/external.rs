use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::State;

use crate::commands::AppError;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn open_path_in_editor(path: String, editor_command: String) -> Result<(), AppError> {
    let resolved = PathBuf::from(&path);
    if !resolved.exists() {
        return Err(AppError::new(format!("Path does not exist: {}", path)));
    }

    let parts = shell_words::split(&editor_command)
        .map_err(|e| AppError::new(format!("Invalid editor command '{}': {}", editor_command, e)))?;
    if parts.is_empty() {
        return Err(AppError::new("Editor command is empty"));
    }

    let program = &parts[0];
    let mut cmd = Command::new(program);
    for arg in &parts[1..] {
        cmd.arg(arg);
    }
    cmd.arg(&path);

    cmd.spawn()
        .map_err(|e| AppError::new(format!("Failed to open editor '{}': {}", editor_command, e)))?;

    Ok(())
}

#[tauri::command]
pub fn reveal_in_finder(path: String) -> Result<(), AppError> {
    let resolved = PathBuf::from(&path);
    if !resolved.exists() {
        return Err(AppError::new(format!("Path does not exist: {}", path)));
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to reveal in Finder: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try xdg-open on the parent directory
        let parent = resolved
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open file manager: {}", e)))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to reveal in Explorer: {}", e)))?;
    }

    Ok(())
}

#[tauri::command]
pub fn open_with_default_app(path: String) -> Result<(), AppError> {
    let resolved = PathBuf::from(&path);
    if !resolved.exists() {
        return Err(AppError::new(format!("Path does not exist: {}", path)));
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open: {}", e)))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| AppError::new(format!("Failed to open: {}", e)))?;
    }

    Ok(())
}

/// Resolve a dropped path (which may be a symlink, a skill folder, or a path
/// inside a skill folder) to a library skill ID (folder name).
#[tauri::command]
pub fn resolve_skill_path(
    path: String,
    state: State<'_, SharedState>,
) -> Result<String, AppError> {
    let input = PathBuf::from(&path);
    if !input.exists() {
        return Err(AppError::new(format!("Path does not exist: {}", path)));
    }

    // Resolve symlinks to their real target
    let canonical = fs::canonicalize(&input)
        .map_err(|e| AppError::new(format!("Cannot resolve path: {}", e)))?;

    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);

    // Check if the canonical path matches a library skill's canonical path
    for skill in &library_skills {
        if let Some(ref skill_canon) = skill.canonical_path {
            let skill_canon_path = PathBuf::from(skill_canon);
            if canonical == skill_canon_path || canonical.starts_with(&skill_canon_path) {
                return Ok(skill.folder_name.clone());
            }
        }
        // Also check the non-canonical path
        let skill_path = PathBuf::from(&skill.path);
        if canonical == skill_path || canonical.starts_with(&skill_path) {
            return Ok(skill.folder_name.clone());
        }
    }

    // Fallback: if the path is a directory with a SKILL.md, use its folder name
    let check_dir = if canonical.is_dir() {
        canonical.clone()
    } else {
        canonical.parent().map(|p| p.to_path_buf()).unwrap_or(canonical.clone())
    };

    if check_dir.join("SKILL.md").exists() {
        if let Some(name) = check_dir.file_name().and_then(|n| n.to_str()) {
            return Ok(name.to_string());
        }
    }

    Err(AppError::new(format!(
        "Could not resolve '{}' to a library skill",
        path
    )))
}
