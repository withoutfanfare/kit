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
        record_linked_locations(&mut assigned_locations, location, &scan.skills);
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

fn record_linked_locations(
    assigned_locations: &mut HashMap<String, Vec<ChangelogAssignedLocation>>,
    location: &SavedLocation,
    assignments: &[SkillAssignment],
) {
    for assignment in assignments
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assignment(skill_id: &str, link_state: LinkState) -> SkillAssignment {
        SkillAssignment {
            skill_id: skill_id.to_string(),
            name: skill_id.to_string(),
            path: String::new(),
            link_state,
            declared_in_manifest: false,
            archived: false,
            source: SkillSource::Library,
            disabled: false,
        }
    }

    #[test]
    fn assigned_locations_include_only_linked_skills_with_saved_location_identity() {
        let alpha = SavedLocation {
            id: "alpha-id".to_string(),
            label: "Alpha".to_string(),
            path: String::new(),
            notes: None,
            last_synced_at: None,
        };
        let beta = SavedLocation {
            id: "beta-id".to_string(),
            label: "Beta".to_string(),
            path: String::new(),
            notes: None,
            last_synced_at: None,
        };
        let alpha_assignments = vec![
            assignment("linked", LinkState::Linked),
            assignment("local", LinkState::LocalOnly),
            assignment("broken", LinkState::BrokenLink),
            assignment("declared", LinkState::DeclaredOnly),
        ];
        let beta_assignments = vec![assignment("linked", LinkState::Linked)];
        let mut result = HashMap::new();

        record_linked_locations(&mut result, &alpha, &alpha_assignments);
        record_linked_locations(&mut result, &beta, &beta_assignments);

        let linked = &result["linked"];
        assert_eq!(linked.len(), 2);
        assert_eq!((&linked[0].id, &linked[0].label), (&alpha.id, &alpha.label));
        assert_eq!((&linked[1].id, &linked[1].label), (&beta.id, &beta.label));
        for excluded in ["local", "broken", "declared"] {
            assert!(!result.contains_key(excluded));
        }
    }
}
