use std::collections::HashMap;
use std::path::Path;

use tauri::State;

use crate::commands::AppError;
use crate::domain::{
    ComparedSkill, ComparisonSide, LocationComparison, SharedSkill, SkillAssignment,
};
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub async fn compare_locations(
    location_a_id: String,
    location_b_id: String,
    state: State<'_, SharedState>,
) -> Result<LocationComparison, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;

    let loc_a = guard
        .find_location(&location_a_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {location_a_id}")))?
        .clone();
    let loc_b = guard
        .find_location(&location_b_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {location_b_id}")))?
        .clone();

    let library_root = guard.preferences().library_root.clone();
    let skill_hashes = guard.inner.skill_hashes.clone();

    drop(guard);

    let lib_path = Path::new(&library_root);
    let library_skills = scanner::scan_library_skills(lib_path);
    let library_sets = scanner::scan_library_sets(lib_path);

    let scan_a = scanner::scan_location(Path::new(&loc_a.path), lib_path, &library_skills, &library_sets);
    let scan_b = scanner::scan_location(Path::new(&loc_b.path), lib_path, &library_skills, &library_sets);

    // Index skills by ID for each location
    let skills_a: HashMap<&str, &SkillAssignment> = scan_a
        .skills
        .iter()
        .map(|s| (s.skill_id.as_str(), s))
        .collect();
    let skills_b: HashMap<&str, &SkillAssignment> = scan_b
        .skills
        .iter()
        .map(|s| (s.skill_id.as_str(), s))
        .collect();

    let mut only_in_a = Vec::new();
    let mut only_in_b = Vec::new();
    let mut shared = Vec::new();

    // Skills only in A, and shared skills
    for (id, skill) in &skills_a {
        if let Some(skill_b) = skills_b.get(id) {
            // Shared — check if version differs via assignment-time hashes
            let key_a = format!("{}:{}", location_a_id, id);
            let key_b = format!("{}:{}", location_b_id, id);
            let hash_a = skill_hashes.get(&key_a).map(|r| r.hash.as_str());
            let hash_b = skill_hashes.get(&key_b).map(|r| r.hash.as_str());
            let version_differs = match (hash_a, hash_b) {
                (Some(a), Some(b)) => a != b,
                _ => false,
            };

            shared.push(SharedSkill {
                skill_id: id.to_string(),
                name: skill.name.clone(),
                link_state_a: skill.link_state.clone(),
                link_state_b: skill_b.link_state.clone(),
                version_differs,
            });
        } else {
            only_in_a.push(ComparedSkill {
                skill_id: id.to_string(),
                name: skill.name.clone(),
                link_state: skill.link_state.clone(),
                source: skill.source.clone(),
            });
        }
    }

    // Skills only in B
    for (id, skill) in &skills_b {
        if !skills_a.contains_key(id) {
            only_in_b.push(ComparedSkill {
                skill_id: id.to_string(),
                name: skill.name.clone(),
                link_state: skill.link_state.clone(),
                source: skill.source.clone(),
            });
        }
    }

    // Sort each list alphabetically
    only_in_a.sort_by(|a, b| a.name.cmp(&b.name));
    only_in_b.sort_by(|a, b| a.name.cmp(&b.name));
    shared.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(LocationComparison {
        location_a: ComparisonSide {
            id: loc_a.id,
            label: loc_a.label,
            path: loc_a.path,
            total_skills: scan_a.skills.len(),
        },
        location_b: ComparisonSide {
            id: loc_b.id,
            label: loc_b.label,
            path: loc_b.path,
            total_skills: scan_b.skills.len(),
        },
        only_in_a,
        only_in_b,
        shared,
    })
}
