use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn update_manifest_entry(
    location_id: String,
    skill_id: String,
    action: String, // "add" or "remove"
    state: State<'_, SharedState>,
) -> Result<LocationDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();

    let location_path = PathBuf::from(&loc.path);
    let manifest_path = location_path.join(".claude").join("settings.json");

    let mut value: serde_json::Value = if manifest_path.is_file() {
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::new(format!("Failed to parse manifest: {}", e)))?
    } else {
        if let Some(parent) = manifest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::new(format!("Failed to create .claude dir: {}", e)))?;
        }
        serde_json::json!({})
    };

    let skills = value
        .as_object_mut()
        .ok_or_else(|| AppError::new("Manifest is not a JSON object"))?
        .entry("skills")
        .or_insert_with(|| serde_json::json!([]))
        .as_array_mut()
        .ok_or_else(|| AppError::new("Manifest 'skills' is not an array"))?;

    match action.as_str() {
        "add" => {
            let already = skills.iter().any(|v| v.as_str() == Some(&skill_id));
            if !already {
                skills.push(serde_json::Value::String(skill_id.clone()));
            }
        }
        "remove" => {
            skills.retain(|v| v.as_str() != Some(&skill_id));
        }
        other => {
            return Err(AppError::new(format!(
                "Invalid action '{}': expected 'add' or 'remove'",
                other
            )));
        }
    }

    let json = serde_json::to_string_pretty(&value)
        .map_err(|e| AppError::new(format!("Failed to serialise manifest: {}", e)))?;
    std::fs::write(&manifest_path, json)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;

    // Re-scan
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    Ok(LocationDetail {
        id: loc.id,
        label: loc.label,
        path: loc.path,
        manifest_path: scan.manifest_path,
        notes: loc.notes,
        sets: scan.sets,
        skills: scan.skills,
        issues: scan.issues,
        stats: scan.stats,
    })
}
