use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::linker;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn run_health_check(
    state: State<'_, SharedState>,
) -> Result<HealthCheckResult, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let locations = guard.locations().to_vec();

    Ok(scanner::run_health_check(
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    ))
}

#[tauri::command]
pub fn fix_broken_links(
    location_id: String,
    state: State<'_, SharedState>,
) -> Result<HealthCheckResult, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let location_path = PathBuf::from(&loc.path);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Remove broken symlinks
    for skill in &scan.skills {
        if skill.link_state == LinkState::BrokenLink && !skill.path.is_empty() {
            let link_path = PathBuf::from(&skill.path);
            let _ = linker::remove_skill_link(&link_path);
        }
    }

    // Re-run the full health check
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let locations = guard.locations().to_vec();

    Ok(scanner::run_health_check(
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    ))
}

/// Read the SKILL.md content for a given skill path.
#[tauri::command]
pub fn read_skill_content(
    skill_path: String,
) -> Result<String, AppError> {
    let path = PathBuf::from(&skill_path);
    let skill_md = path.join("SKILL.md");
    if !skill_md.is_file() {
        return Err(AppError::new(format!(
            "SKILL.md not found at: {}",
            skill_md.display()
        )));
    }
    let content = std::fs::read_to_string(&skill_md)
        .map_err(|e| AppError::new(format!("Failed to read SKILL.md: {}", e)))?;
    Ok(content)
}

/// Get skill version info for a location — compares current hashes against
/// recorded assignment hashes.
#[tauri::command]
pub fn get_skill_versions(
    location_id: String,
    state: State<'_, SharedState>,
) -> Result<Vec<SkillVersionInfo>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let location_path = PathBuf::from(&loc.path);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    let mut versions = Vec::new();

    for skill in &scan.skills {
        if skill.link_state != LinkState::Linked {
            continue;
        }

        let key = format!("{}:{}", location_id, skill.skill_id);
        let recorded = guard.inner.skill_hashes.get(&key);
        let current_hash = scanner::hash_skill_content(&library_root.join(&skill.skill_id));

        let has_changed = match (recorded, &current_hash) {
            (Some(rec), Some(curr)) => rec.hash != *curr,
            (None, Some(_)) => false, // No recorded hash — can't tell if changed
            _ => false,
        };

        versions.push(SkillVersionInfo {
            skill_id: skill.skill_id.clone(),
            assigned_hash: recorded.map(|r| r.hash.clone()),
            current_hash,
            has_changed,
            assigned_at: recorded.and_then(|r| r.assigned_at),
        });
    }

    Ok(versions)
}
