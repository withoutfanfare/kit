use std::{collections::HashMap, path::PathBuf};
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

/// List recently modified skills in the library, sorted by modification time
/// (most recent first). Optionally filter to skills modified within `days`.
#[tauri::command]
pub fn get_skill_changelog(
    days: Option<u64>,
    state: State<'_, SharedState>,
) -> Result<Vec<ChangelogEntry>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let locations = guard.locations().to_vec();
    drop(guard);

    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let mut assigned_locations: HashMap<String, Vec<ChangelogAssignedLocation>> = HashMap::new();
    for location in &locations {
        let scan = scanner::scan_location(
            &PathBuf::from(&location.path),
            &library_root,
            &library_skills,
            &library_sets,
        );

        for assignment in scan
            .skills
            .iter()
            .filter(|assignment| assignment.link_state == LinkState::Linked)
        {
            assigned_locations
                .entry(assignment.skill_id.clone())
                .or_default()
                .push(ChangelogAssignedLocation {
                    id: location.id.clone(),
                    label: location.label.clone(),
                });
        }
    }

    let cutoff = days.map(|d| chrono::Utc::now() - chrono::Duration::days(d as i64));

    let mut entries: Vec<ChangelogEntry> = Vec::new();

    for skill in &library_skills {
        let skill_dir = library_root.join(&skill.folder_name);
        let skill_md = skill_dir.join("SKILL.md");

        let metadata = match std::fs::metadata(&skill_md) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified: chrono::DateTime<chrono::Utc> = match metadata.modified() {
            Ok(t) => t.into(),
            Err(_) => continue,
        };

        if let Some(cutoff) = cutoff {
            if modified < cutoff {
                continue;
            }
        }

        entries.push(ChangelogEntry {
            skill_id: skill.folder_name.clone(),
            name: skill.name.clone(),
            modified_at: modified,
            size_bytes: metadata.len(),
            assigned_locations: assigned_locations
                .remove(&skill.folder_name)
                .unwrap_or_default(),
        });
    }

    // Sort by most recently modified first
    entries.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    Ok(entries)
}
