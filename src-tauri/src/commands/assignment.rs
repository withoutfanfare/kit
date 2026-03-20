use std::path::{Path, PathBuf};
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::linker;
use crate::scanner;
use crate::state::{self, SharedState};

/// Reject skill IDs that contain path traversal characters.
fn validate_skill_ids(ids: &[String]) -> Result<(), AppError> {
    for sid in ids {
        if sid.contains('/') || sid.contains('\\') || sid.contains("..") {
            return Err(AppError::new(format!("Invalid skill ID: {}", sid)));
        }
    }
    Ok(())
}

#[tauri::command]
pub fn preview_assignment(
    location_id: String,
    skill_ids_to_add: Vec<String>,
    set_ids_to_add: Vec<String>,
    skill_ids_to_remove: Vec<String>,
    set_ids_to_remove: Vec<String>,
    state: State<'_, SharedState>,
) -> Result<AssignmentPreview, AppError> {
    validate_skill_ids(&skill_ids_to_add)?;
    validate_skill_ids(&skill_ids_to_remove)?;

    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?;

    let location_path = PathBuf::from(&loc.path);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let project_sets = scanner::scan_project_sets(&location_path);
    let all_sets: Vec<(String, SetDefinition)> = library_sets
        .iter()
        .chain(project_sets.iter())
        .cloned()
        .collect();
    let scan = scanner::scan_location(
        &location_path,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Read the current manifest sets to know what's already assigned
    let manifest_path = location_path.join(".claude").join("settings.json");
    let current_manifest_sets = scanner::read_manifest_sets(&manifest_path);

    // Expand sets to add into skill IDs
    let mut expanded_skill_ids = skill_ids_to_add.clone();
    for set_id in &set_ids_to_add {
        if let Some((_, set_def)) = all_sets.iter().find(|(id, _)| id == set_id) {
            for skill_id in &set_def.skills {
                if !expanded_skill_ids.contains(skill_id) {
                    expanded_skill_ids.push(skill_id.clone());
                }
            }
        }
    }

    // Determine which skills are still needed by remaining sets after removal
    let remaining_set_ids: Vec<String> = current_manifest_sets
        .iter()
        .chain(set_ids_to_add.iter())
        .filter(|id| !set_ids_to_remove.contains(id))
        .cloned()
        .collect();

    let skills_needed_by_remaining_sets: Vec<String> = remaining_set_ids
        .iter()
        .flat_map(|sid| {
            all_sets
                .iter()
                .find(|(id, _)| id == sid)
                .map(|(_, def)| def.skills.clone())
                .unwrap_or_default()
        })
        .collect();

    // Expand sets to remove: find skills that should be unlinked
    let mut expanded_skill_removals = skill_ids_to_remove.clone();
    for set_id in &set_ids_to_remove {
        if let Some((_, set_def)) = all_sets.iter().find(|(id, _)| id == set_id) {
            for skill_id in &set_def.skills {
                // Only remove if no other remaining set or explicit skill needs it
                let needed_by_set = skills_needed_by_remaining_sets.contains(skill_id);
                let is_explicit_skill = scanner::read_manifest_skills(&manifest_path)
                    .contains(skill_id);
                if !needed_by_set && !is_explicit_skill && !expanded_skill_removals.contains(skill_id) {
                    expanded_skill_removals.push(skill_id.clone());
                }
            }
        }
    }

    let mut adds = Vec::new();
    let mut removes = Vec::new();
    let mut manifest_updates = Vec::new();
    let mut warnings = Vec::new();

    // Preview additions
    for sid in &expanded_skill_ids {
        let skill = library_skills.iter().find(|s| s.folder_name == *sid);
        let name = skill.map(|s| s.name.clone()).unwrap_or_else(|| sid.clone());

        let already_linked = scan
            .skills
            .iter()
            .any(|s| s.skill_id == *sid && s.link_state == LinkState::Linked);

        if already_linked {
            warnings.push(format!("'{}' is already linked", name));
            continue;
        }

        if skill.is_none() {
            warnings.push(format!("Skill '{}' not found in library", sid));
            continue;
        }

        adds.push(PreviewChange {
            kind: PreviewChangeKind::AddLink,
            skill_name: name.clone(),
            detail: format!("Create symlink for '{}'", name),
        });

        // Check manifest
        let declared = scan
            .skills
            .iter()
            .any(|s| s.skill_id == *sid && s.declared_in_manifest);
        if !declared {
            manifest_updates.push(PreviewChange {
                kind: PreviewChangeKind::ManifestAdd,
                skill_name: name.clone(),
                detail: format!("Add '{}' to manifest", sid),
            });
        }
    }

    // Preview set manifest additions
    for set_id in &set_ids_to_add {
        if !current_manifest_sets.contains(set_id) {
            let set_name = all_sets
                .iter()
                .find(|(id, _)| id == set_id)
                .map(|(_, def)| def.name.clone())
                .unwrap_or_else(|| set_id.clone());
            manifest_updates.push(PreviewChange {
                kind: PreviewChangeKind::ManifestAdd,
                skill_name: set_name,
                detail: format!("Add set '{}' to manifest", set_id),
            });
        }
    }

    // Preview set manifest removals
    for set_id in &set_ids_to_remove {
        if current_manifest_sets.contains(set_id) {
            let set_name = all_sets
                .iter()
                .find(|(id, _)| id == set_id)
                .map(|(_, def)| def.name.clone())
                .unwrap_or_else(|| set_id.clone());
            manifest_updates.push(PreviewChange {
                kind: PreviewChangeKind::ManifestRemove,
                skill_name: set_name,
                detail: format!("Remove set '{}' from manifest", set_id),
            });
        }
    }

    // Preview skill removals
    for sid in &expanded_skill_removals {
        let assignment = scan.skills.iter().find(|s| s.skill_id == *sid);
        let name = assignment
            .map(|s| s.name.clone())
            .unwrap_or_else(|| sid.clone());

        match assignment {
            Some(a) if a.link_state == LinkState::Linked || a.link_state == LinkState::BrokenLink => {
                removes.push(PreviewChange {
                    kind: PreviewChangeKind::RemoveLink,
                    skill_name: name.clone(),
                    detail: format!("Remove symlink for '{}'", name),
                });

                if a.declared_in_manifest {
                    manifest_updates.push(PreviewChange {
                        kind: PreviewChangeKind::ManifestRemove,
                        skill_name: name.clone(),
                        detail: format!("Remove '{}' from manifest", sid),
                    });
                }
            }
            Some(_) => {
                warnings.push(format!("'{}' is not a library symlink, cannot remove", name));
            }
            None => {
                warnings.push(format!("'{}' is not installed at this location", sid));
            }
        }
    }

    Ok(AssignmentPreview {
        location_id,
        adds,
        removes,
        manifest_updates,
        warnings,
    })
}

#[tauri::command]
pub fn apply_assignment(
    location_id: String,
    skill_ids_to_add: Vec<String>,
    set_ids_to_add: Vec<String>,
    skill_ids_to_remove: Vec<String>,
    set_ids_to_remove: Vec<String>,
    update_manifest: bool,
    state: State<'_, SharedState>,
) -> Result<LocationDetail, AppError> {
    validate_skill_ids(&skill_ids_to_add)?;
    validate_skill_ids(&skill_ids_to_remove)?;

    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();

    let location_path = PathBuf::from(&loc.path);

    // Gather all sets (global + project)
    let library_skills_for_sets = scanner::scan_library_skills(&library_root);
    let library_sets_for_expand = scanner::scan_library_sets(&library_root);
    let project_sets = scanner::scan_project_sets(&location_path);
    let all_sets: Vec<(String, SetDefinition)> = library_sets_for_expand
        .iter()
        .chain(project_sets.iter())
        .cloned()
        .collect();

    // Read current manifest sets
    let manifest_path = location_path.join(".claude").join("settings.json");
    let current_manifest_sets = scanner::read_manifest_sets(&manifest_path);
    let current_manifest_skills = scanner::read_manifest_skills(&manifest_path);

    // Expand sets to add into skill IDs
    let mut expanded_skill_ids = skill_ids_to_add.clone();
    for set_id in &set_ids_to_add {
        if let Some((_, set_def)) = all_sets.iter().find(|(id, _)| id == set_id) {
            for skill_id in &set_def.skills {
                if !expanded_skill_ids.contains(skill_id) {
                    expanded_skill_ids.push(skill_id.clone());
                }
            }
        }
    }

    // Determine which sets will remain after this operation
    let remaining_set_ids: Vec<String> = current_manifest_sets
        .iter()
        .chain(set_ids_to_add.iter())
        .filter(|id| !set_ids_to_remove.contains(id))
        .cloned()
        .collect();

    // Skills needed by the remaining sets
    let skills_needed_by_remaining_sets: Vec<String> = remaining_set_ids
        .iter()
        .flat_map(|sid| {
            all_sets
                .iter()
                .find(|(id, _)| id == sid)
                .map(|(_, def)| def.skills.clone())
                .unwrap_or_default()
        })
        .collect();

    // Expand sets to remove: unlink skills no longer needed
    let mut expanded_skill_removals = skill_ids_to_remove.clone();
    for set_id in &set_ids_to_remove {
        if let Some((_, set_def)) = all_sets.iter().find(|(id, _)| id == set_id) {
            for skill_id in &set_def.skills {
                let needed_by_set = skills_needed_by_remaining_sets.contains(skill_id);
                let is_explicit_skill = current_manifest_skills.contains(skill_id);
                if !needed_by_set && !is_explicit_skill && !expanded_skill_removals.contains(skill_id) {
                    expanded_skill_removals.push(skill_id.clone());
                }
            }
        }
    }

    // Drop temporary variables
    drop(library_skills_for_sets);
    drop(library_sets_for_expand);
    drop(project_sets);
    drop(all_sets);

    // Perform skill removals
    for sid in &expanded_skill_removals {
        let skills_dir = scanner::find_skills_dir(&location_path);
        if let Some(sd) = skills_dir {
            let link_path = sd.join(sid);
            if std::fs::symlink_metadata(&link_path).is_ok() {
                linker::remove_skill_link(&link_path).map_err(AppError::new)?;
            }
        }
    }

    // Perform skill additions
    if !expanded_skill_ids.is_empty() {
        let skills_dir = linker::ensure_skills_dir(&location_path).map_err(AppError::new)?;
        for sid in &expanded_skill_ids {
            let target = library_root.join(sid);
            let link_path = skills_dir.join(sid);

            // Skip if already exists
            if std::fs::symlink_metadata(&link_path).is_ok() {
                continue;
            }

            linker::create_skill_link(&target, &link_path).map_err(AppError::new)?;
        }
    }

    // Optionally update the manifest
    if update_manifest {
        update_manifest_skills_and_sets(
            &location_path,
            &expanded_skill_ids,
            &expanded_skill_removals,
            &set_ids_to_add,
            &set_ids_to_remove,
        )?;
    }

    // Update last synced
    if let Some(loc_mut) = guard.find_location_mut(&location_id) {
        loc_mut.last_synced_at = Some(chrono::Utc::now());
    }

    // Record skill content hashes for version tracking
    if guard.inner.preferences.track_skill_versions {
        for sid in &expanded_skill_ids {
            let skill_path = library_root.join(sid);
            if let Some(hash) = scanner::hash_skill_content(&skill_path) {
                let key = format!("{}:{}", location_id, sid);
                guard.inner.skill_hashes.insert(
                    key,
                    state::SkillHashRecord {
                        hash,
                        assigned_at: Some(chrono::Utc::now()),
                    },
                );
            }
        }
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

    let assigned_ids: Vec<String> = scan.skills.iter().map(|s| s.skill_id.clone()).collect();
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
        skills: scan.skills,
        issues: scan.issues,
        stats: scan.stats,
        detected_project_types: scan.detected_project_types,
        skill_recommendations,
        last_scanned_at: Some(chrono::Utc::now()),
    })
}

/// Helper to update the manifest's `skills` and `sets` arrays.
fn update_manifest_skills_and_sets(
    location_path: &Path,
    skills_to_add: &[String],
    skills_to_remove: &[String],
    sets_to_add: &[String],
    sets_to_remove: &[String],
) -> Result<(), AppError> {
    let manifest_path = location_path.join(".claude").join("settings.json");

    let mut value: serde_json::Value = if manifest_path.is_file() {
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::new(format!("Failed to parse manifest: {}", e)))?
    } else {
        // Create new manifest
        if let Some(parent) = manifest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::new(format!("Failed to create .claude dir: {}", e)))?;
        }
        serde_json::json!({})
    };

    let obj = value
        .as_object_mut()
        .ok_or_else(|| AppError::new("Manifest is not a JSON object"))?;

    // Update skills array
    let skills = obj
        .entry("skills")
        .or_insert_with(|| serde_json::json!([]))
        .as_array_mut()
        .ok_or_else(|| AppError::new("Manifest 'skills' is not an array"))?;

    skills.retain(|v| {
        v.as_str()
            .map(|s| !skills_to_remove.contains(&s.to_string()))
            .unwrap_or(true)
    });

    for sid in skills_to_add {
        let already = skills.iter().any(|v| v.as_str() == Some(sid.as_str()));
        if !already {
            skills.push(serde_json::Value::String(sid.clone()));
        }
    }

    // Update sets array
    let sets = obj
        .entry("sets")
        .or_insert_with(|| serde_json::json!([]))
        .as_array_mut()
        .ok_or_else(|| AppError::new("Manifest 'sets' is not an array"))?;

    sets.retain(|v| {
        v.as_str()
            .map(|s| !sets_to_remove.contains(&s.to_string()))
            .unwrap_or(true)
    });

    for sid in sets_to_add {
        let already = sets.iter().any(|v| v.as_str() == Some(sid.as_str()));
        if !already {
            sets.push(serde_json::Value::String(sid.clone()));
        }
    }

    let json = serde_json::to_string_pretty(&value)
        .map_err(|e| AppError::new(format!("Failed to serialise manifest: {}", e)))?;
    state::atomic_write(&manifest_path, &json)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;

    Ok(())
}
