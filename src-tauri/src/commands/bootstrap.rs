use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn get_app_bootstrap(state: State<'_, SharedState>) -> Result<AppBootstrap, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let locations: Vec<SavedLocationSummary> = guard
        .locations()
        .iter()
        .map(|loc| {
            scanner::build_location_summary(loc, &library_root, &library_skills, &library_sets)
        })
        .collect();

    let broken_links: usize = locations.iter().map(|l| l.issue_count).sum();

    let archived_skills = library_skills.iter().filter(|s| s.archived).count();

    Ok(AppBootstrap {
        library_root: prefs.library_root.clone(),
        editor_command: prefs.editor_command.clone(),
        default_view: prefs.default_view.clone(),
        show_archived: prefs.show_archived,
        locations,
        counts: BootstrapCounts {
            skills: library_skills.len(),
            sets: library_sets.len(),
            archived_skills,
            broken_links,
        },
    })
}

#[tauri::command]
pub fn update_preferences(
    prefs: PreferencesUpdate,
    state: State<'_, SharedState>,
) -> Result<Preferences, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let current = &mut guard.inner.preferences;
    if let Some(root) = &prefs.library_root {
        if !root.is_empty() {
            let path = PathBuf::from(root);
            if !path.is_dir() {
                return Err(AppError::new(format!(
                    "Library root does not exist: {}",
                    root
                )));
            }
        }
        current.library_root = root.clone();
    }
    if let Some(cmd) = prefs.editor_command {
        current.editor_command = cmd;
    }
    if let Some(view) = prefs.default_view {
        current.default_view = view;
    }
    if let Some(archived) = prefs.show_archived {
        current.show_archived = archived;
    }
    if let Some(track) = prefs.track_skill_versions {
        current.track_skill_versions = track;
    }

    let result = current.clone();
    guard.save().map_err(AppError::new)?;
    Ok(result)
}

#[tauri::command]
pub fn get_app_data_path() -> Result<String, AppError> {
    let home = dirs::home_dir()
        .ok_or_else(|| AppError::new("Cannot determine home directory"))?;
    let path = home.join(".kit");
    Ok(path.to_string_lossy().to_string())
}
