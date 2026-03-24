use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

/// Toggle a skill's activation state at a specific location.
/// Disabling removes the skill from the manifest (so Claude won't load it)
/// but keeps the symlink in place. Enabling adds it back.
#[tauri::command]
pub fn toggle_skill_activation(
    location_id: String,
    skill_id: String,
    state: State<'_, SharedState>,
) -> Result<LocationDetail, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();

    let location_path = PathBuf::from(&loc.path);
    let key = format!("{}:{}", location_id, skill_id);

    let is_currently_disabled = guard.inner.disabled_skills.contains(&key);

    if is_currently_disabled {
        // Re-enable: remove from disabled set, add to manifest
        guard.inner.disabled_skills.remove(&key);
        add_skill_to_manifest(&location_path, &skill_id)?;
    } else {
        // Disable: add to disabled set, remove from manifest
        guard.inner.disabled_skills.insert(key);
        remove_skill_from_manifest(&location_path, &skill_id)?;
    }

    guard.save().map_err(AppError::new)?;

    // Re-scan to build updated detail
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Enrich disabled state
    let mut skills = scan.skills;
    for skill in &mut skills {
        let sk = format!("{}:{}", location_id, skill.skill_id);
        skill.disabled = guard.inner.disabled_skills.contains(&sk);
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
        last_scanned_at: Some(chrono::Utc::now()),
    })
}

/// Validate the body content of a SKILL.md file at the given path.
#[tauri::command]
pub fn get_skill_body_validation(
    skill_path: String,
) -> Result<Vec<ValidationIssue>, AppError> {
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
    Ok(scanner::validate_skill_body(&content))
}

/// Get a diff between the SKILL.md content at assignment time and its
/// current content, for a specific skill at a specific location.
#[tauri::command]
pub fn get_skill_content_diff(
    location_id: String,
    skill_id: String,
    state: State<'_, SharedState>,
) -> Result<SkillContentDiff, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let key = format!("{}:{}", location_id, skill_id);
    let assigned_content = guard.inner.skill_snapshots.get(&key).cloned();

    // Read current content
    let skill_md = library_root.join(&skill_id).join("SKILL.md");
    let current_content = std::fs::read_to_string(&skill_md).ok();

    let has_changed = match (&assigned_content, &current_content) {
        (Some(a), Some(c)) => a != c,
        (None, Some(_)) => false, // No snapshot — can't determine change
        _ => false,
    };

    Ok(SkillContentDiff {
        skill_id,
        assigned_content,
        current_content,
        has_changed,
    })
}

// ---------------------------------------------------------------------------
// Manifest helpers (skill-level add/remove without touching sets)
// ---------------------------------------------------------------------------

fn add_skill_to_manifest(
    location_path: &std::path::Path,
    skill_id: &str,
) -> Result<(), AppError> {
    let manifest_path = location_path.join(".claude").join("settings.json");

    let mut value: serde_json::Value = if manifest_path.is_file() {
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::new(format!("Failed to parse manifest: {}", e)))?
    } else {
        return Ok(()); // No manifest, nothing to update
    };

    let obj = value
        .as_object_mut()
        .ok_or_else(|| AppError::new("Manifest is not a JSON object"))?;

    let skills = obj
        .entry("skills")
        .or_insert_with(|| serde_json::json!([]))
        .as_array_mut()
        .ok_or_else(|| AppError::new("Manifest 'skills' is not an array"))?;

    let already = skills.iter().any(|v| v.as_str() == Some(skill_id));
    if !already {
        skills.push(serde_json::Value::String(skill_id.to_string()));
    }

    let json = serde_json::to_string_pretty(&value)
        .map_err(|e| AppError::new(format!("Failed to serialise manifest: {}", e)))?;
    crate::state::atomic_write(&manifest_path, &json)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;
    Ok(())
}

fn remove_skill_from_manifest(
    location_path: &std::path::Path,
    skill_id: &str,
) -> Result<(), AppError> {
    let manifest_path = location_path.join(".claude").join("settings.json");

    if !manifest_path.is_file() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&manifest_path)
        .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
    let mut value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| AppError::new(format!("Failed to parse manifest: {}", e)))?;

    if let Some(obj) = value.as_object_mut() {
        if let Some(serde_json::Value::Array(skills)) = obj.get_mut("skills") {
            skills.retain(|v| v.as_str() != Some(skill_id));
        }
    }

    let json = serde_json::to_string_pretty(&value)
        .map_err(|e| AppError::new(format!("Failed to serialise manifest: {}", e)))?;
    crate::state::atomic_write(&manifest_path, &json)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;
    Ok(())
}
