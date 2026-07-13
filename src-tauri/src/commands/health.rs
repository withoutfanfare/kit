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
    let locations = guard.locations().to_vec();
    drop(guard);

    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    Ok(scanner::run_health_check(
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    ))
}

fn health_inputs(state: State<'_, SharedState>) -> Result<(String, Vec<SavedLocation>), AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    Ok((
        guard.preferences().library_root.clone(),
        guard.locations().to_vec(),
    ))
}

fn selected_locations<'a>(
    location_ids: &[String],
    locations: &'a [SavedLocation],
) -> Result<Vec<&'a SavedLocation>, AppError> {
    for location_id in location_ids {
        if !locations.iter().any(|location| location.id == *location_id) {
            return Err(AppError::new(format!(
                "Location not found: {location_id}"
            )));
        }
    }

    Ok(locations
        .iter()
        .filter(|location| location_ids.contains(&location.id))
        .collect())
}

fn broken_link_paths(
    location: &SavedLocation,
    library_root: &std::path::Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Vec<PathBuf> {
    let scan = scanner::scan_location(
        &PathBuf::from(&location.path),
        library_root,
        library_skills,
        library_sets,
    );

    scan.skills
        .into_iter()
        .filter(|skill| skill.link_state == LinkState::BrokenLink && !skill.path.is_empty())
        .map(|skill| PathBuf::from(skill.path))
        .collect()
}

fn preview_broken_link_removal_for_locations(
    location_ids: &[String],
    locations: &[SavedLocation],
    library_root: &std::path::Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Result<Vec<BrokenLinkRemovalPreview>, AppError> {
    Ok(selected_locations(location_ids, locations)?
        .into_iter()
        .filter_map(|location| {
            let paths = broken_link_paths(
                location,
                library_root,
                library_skills,
                library_sets,
            );
            (!paths.is_empty()).then(|| BrokenLinkRemovalPreview {
                location_id: location.id.clone(),
                location_label: location.label.clone(),
                paths: paths
                    .into_iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect(),
            })
        })
        .collect())
}

fn remove_broken_links_for_locations(
    location_ids: &[String],
    locations: &[SavedLocation],
    library_root: &std::path::Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Result<BrokenLinkRemovalResult, AppError> {
    let mut removed = 0;
    for location in selected_locations(location_ids, locations)? {
        for path in broken_link_paths(
            location,
            library_root,
            library_skills,
            library_sets,
        ) {
            let is_broken_symlink = std::fs::symlink_metadata(&path)
                .map(|metadata| metadata.file_type().is_symlink() && !path.exists())
                .unwrap_or(false);
            if is_broken_symlink {
                linker::remove_skill_link(&path).map_err(AppError::new)?;
                removed += 1;
            }
        }
    }
    Ok(BrokenLinkRemovalResult {
        removed_count: removed,
        health: scanner::run_health_check(
            locations,
            library_root,
            library_skills,
            library_sets,
        ),
    })
}

#[tauri::command]
pub fn preview_broken_link_removal(
    location_ids: Vec<String>,
    state: State<'_, SharedState>,
) -> Result<Vec<BrokenLinkRemovalPreview>, AppError> {
    let (library_root, locations) = health_inputs(state)?;
    let library_root = PathBuf::from(library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    preview_broken_link_removal_for_locations(
        &location_ids,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    )
}

#[tauri::command]
pub fn remove_broken_links(
    location_ids: Vec<String>,
    state: State<'_, SharedState>,
) -> Result<BrokenLinkRemovalResult, AppError> {
    let (library_root, locations) = health_inputs(state)?;
    let library_root = PathBuf::from(library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    remove_broken_links_for_locations(
        &location_ids,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    )
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
    let skill_hashes = guard.inner.skill_hashes.clone();

    let loc = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();
    drop(guard);

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

    let mut versions = Vec::new();

    for skill in &scan.skills {
        if skill.link_state != LinkState::Linked {
            continue;
        }

        let key = format!("{}:{}", location_id, skill.skill_id);
        let recorded = skill_hashes.get(&key);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn apply_ignores_a_link_repaired_after_preview() {
        use std::fs;
        use std::os::unix::fs::symlink;

        let base = std::env::temp_dir().join(format!(
            "kit-health-stale-preview-test-{}",
            std::process::id()
        ));
        fs::remove_dir_all(&base).ok();
        let library_root = base.join("library");
        let location_path = base.join("location");
        let skills_path = location_path.join(".claude/skills");
        let target_path = base.join("repaired-target");
        let link_path = skills_path.join("repaired-link");
        fs::create_dir_all(&library_root).unwrap();
        fs::create_dir_all(&skills_path).unwrap();
        symlink(&target_path, &link_path).unwrap();

        let locations = vec![SavedLocation {
            id: "location-1".to_string(),
            label: "Location 1".to_string(),
            path: location_path.to_string_lossy().to_string(),
            notes: None,
            last_synced_at: None,
        }];
        let location_ids = vec!["location-1".to_string()];

        let preview = preview_broken_link_removal_for_locations(
            &location_ids,
            &locations,
            &library_root,
            &[],
            &[],
        )
        .unwrap();
        assert_eq!(preview[0].paths, vec![link_path.to_string_lossy()]);

        fs::create_dir_all(&target_path).unwrap();
        let result = remove_broken_links_for_locations(
            &location_ids,
            &locations,
            &library_root,
            &[],
            &[],
        )
        .unwrap();

        assert!(fs::symlink_metadata(&link_path)
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(
            serde_json::to_value(result).unwrap()["removedCount"],
            serde_json::json!(0)
        );

        fs::remove_dir_all(&base).ok();
    }
}
