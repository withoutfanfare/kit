use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn list_locations(state: State<'_, SharedState>) -> Result<Vec<SavedLocationSummary>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let summaries: Vec<SavedLocationSummary> = guard
        .locations()
        .iter()
        .map(|loc| {
            scanner::build_location_summary(loc, &library_root, &library_skills, &library_sets)
        })
        .collect();

    Ok(summaries)
}

#[tauri::command]
pub fn add_location(
    label: String,
    path: String,
    state: State<'_, SharedState>,
) -> Result<SavedLocationSummary, AppError> {
    let resolved = PathBuf::from(&path);
    if !resolved.is_dir() {
        return Err(AppError::new(format!(
            "Path is not a valid directory: {}",
            path
        )));
    }

    let canonical = std::fs::canonicalize(&resolved)
        .map_err(|e| AppError::new(format!("Cannot resolve path: {}", e)))?;
    let canonical_str = canonical.to_string_lossy().to_string();

    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    // Check for duplicate path
    if guard.locations().iter().any(|l| l.path == canonical_str) {
        return Err(AppError::new(format!(
            "Location already added: {}",
            canonical_str
        )));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let loc = SavedLocation {
        id: id.clone(),
        label: if label.is_empty() {
            canonical
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string()
        } else {
            label
        },
        path: canonical_str,
        notes: None,
        last_synced_at: Some(chrono::Utc::now()),
    };

    guard.locations_mut().push(loc.clone());
    guard.save().map_err(AppError::new)?;

    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    Ok(scanner::build_location_summary(
        &loc,
        &library_root,
        &library_skills,
        &library_sets,
    ))
}

#[tauri::command]
pub fn update_location(
    id: String,
    label: Option<String>,
    notes: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SavedLocationSummary, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let loc = guard
        .find_location_mut(&id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", id)))?;

    if let Some(l) = label {
        loc.label = l;
    }
    if let Some(n) = notes {
        loc.notes = Some(n);
    }

    let loc_snapshot = loc.clone();
    guard.save().map_err(AppError::new)?;

    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    Ok(scanner::build_location_summary(
        &loc_snapshot,
        &library_root,
        &library_skills,
        &library_sets,
    ))
}

#[tauri::command]
pub fn remove_location(
    id: String,
    state: State<'_, SharedState>,
) -> Result<Vec<SavedLocationSummary>, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let before_len = guard.locations().len();
    guard.locations_mut().retain(|l| l.id != id);
    if guard.locations().len() == before_len {
        return Err(AppError::new(format!("Location not found: {}", id)));
    }

    guard.save().map_err(AppError::new)?;

    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let summaries: Vec<SavedLocationSummary> = guard
        .locations()
        .iter()
        .map(|loc| {
            scanner::build_location_summary(loc, &library_root, &library_skills, &library_sets)
        })
        .collect();

    Ok(summaries)
}

#[tauri::command]
pub fn get_location_detail(
    id: String,
    state: State<'_, SharedState>,
) -> Result<LocationDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let loc = guard
        .find_location(&id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", id)))?
        .clone();

    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let location_path = PathBuf::from(&loc.path);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Enrich disabled state from persisted state
    let mut skills = scan.skills;
    for skill in &mut skills {
        let key = format!("{}:{}", id, skill.skill_id);
        skill.disabled = guard.inner.disabled_skills.contains(&key);
    }

    let assigned_ids: Vec<String> = skills.iter().map(|s| s.skill_id.clone()).collect();
    let skill_recommendations = scanner::recommend_skills(
        &scan.detected_project_types,
        &library_skills,
        &assigned_ids,
    );

    Ok(LocationDetail {
        id: loc.id,
        label: loc.label,
        path: loc.path,
        manifest_path: scan.manifest_path,
        notes: loc.notes,
        sets: scan.sets,
        skills,
        issues: scan.issues,
        stats: scan.stats,
        detected_project_types: scan.detected_project_types,
        skill_recommendations,
        last_scanned_at: loc.last_synced_at,
    })
}

#[tauri::command]
pub fn sync_location(
    id: String,
    state: State<'_, SharedState>,
) -> Result<LocationDetail, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let loc = guard
        .find_location_mut(&id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", id)))?;

    loc.last_synced_at = Some(chrono::Utc::now());
    let loc_snapshot = loc.clone();
    guard.save().map_err(AppError::new)?;

    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let location_path = PathBuf::from(&loc_snapshot.path);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Enrich disabled state from persisted state
    let mut skills = scan.skills;
    for skill in &mut skills {
        let key = format!("{}:{}", id, skill.skill_id);
        skill.disabled = guard.inner.disabled_skills.contains(&key);
    }

    let assigned_ids: Vec<String> = skills.iter().map(|s| s.skill_id.clone()).collect();
    let skill_recommendations = scanner::recommend_skills(
        &scan.detected_project_types,
        &library_skills,
        &assigned_ids,
    );

    Ok(LocationDetail {
        id: loc_snapshot.id,
        label: loc_snapshot.label,
        path: loc_snapshot.path,
        manifest_path: scan.manifest_path,
        notes: loc_snapshot.notes,
        sets: scan.sets,
        skills,
        issues: scan.issues,
        stats: scan.stats,
        detected_project_types: scan.detected_project_types,
        skill_recommendations,
        last_scanned_at: loc_snapshot.last_synced_at,
    })
}
