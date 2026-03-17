use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn list_library_items(
    state: State<'_, SharedState>,
) -> Result<Vec<LibraryListItem>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let mut items: Vec<LibraryListItem> = Vec::new();

    // Add skills
    for skill in &library_skills {
        let linked_count = guard
            .locations()
            .iter()
            .filter(|loc| {
                let loc_path = PathBuf::from(&loc.path);
                let scan = scanner::scan_location(
                    &loc_path,
                    &library_root,
                    &library_skills,
                    &library_sets,
                );
                scan.skills
                    .iter()
                    .any(|s| s.skill_id == skill.folder_name && s.link_state == LinkState::Linked)
            })
            .count();

        items.push(LibraryListItem {
            id: skill.folder_name.clone(),
            name: skill.name.clone(),
            kind: LibraryItemKind::Skill,
            archived: skill.archived,
            summary: skill.description.clone(),
            linked_location_count: linked_count,
        });
    }

    // Add sets
    for (set_id, set_def) in &library_sets {
        let linked_count = guard
            .locations()
            .iter()
            .filter(|loc| {
                let loc_path = PathBuf::from(&loc.path);
                let scan = scanner::scan_location(
                    &loc_path,
                    &library_root,
                    &library_skills,
                    &library_sets,
                );
                // A set is considered "linked" to a location if any of its
                // skills are linked there.
                set_def.skills.iter().any(|sid| {
                    scan.skills
                        .iter()
                        .any(|s| s.skill_id == *sid && s.link_state == LinkState::Linked)
                })
            })
            .count();

        items.push(LibraryListItem {
            id: set_id.clone(),
            name: set_def.name.clone(),
            kind: LibraryItemKind::Set,
            archived: false,
            summary: set_def.description.clone(),
            linked_location_count: linked_count,
        });
    }

    Ok(items)
}

#[tauri::command]
pub fn get_skill_detail(
    skill_id: String,
    state: State<'_, SharedState>,
) -> Result<SkillDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let skill = library_skills
        .iter()
        .find(|s| s.folder_name == skill_id)
        .ok_or_else(|| AppError::new(format!("Skill not found: {}", skill_id)))?;

    let linked_locations = scanner::locations_linking_skill(
        &skill_id,
        guard.locations(),
        &library_root,
        &library_skills,
        &library_sets,
    );

    let included_in_sets: Vec<SetRef> = library_sets
        .iter()
        .filter(|(_, def)| def.skills.contains(&skill_id))
        .map(|(id, def)| SetRef {
            id: id.clone(),
            name: def.name.clone(),
        })
        .collect();

    let usage = scanner::skill_usage(&skill_id, &guard.inner.usage);

    Ok(SkillDetail {
        id: skill.folder_name.clone(),
        name: skill.name.clone(),
        path: skill.path.clone(),
        archived: skill.archived,
        summary: skill.description.clone(),
        linked_locations,
        included_in_sets,
        usage,
    })
}

#[tauri::command]
pub fn archive_skill(
    skill_id: String,
    state: State<'_, SharedState>,
) -> Result<SkillDetail, AppError> {
    set_skill_archived(&skill_id, true, state)
}

#[tauri::command]
pub fn unarchive_skill(
    skill_id: String,
    state: State<'_, SharedState>,
) -> Result<SkillDetail, AppError> {
    set_skill_archived(&skill_id, false, state)
}

fn set_skill_archived(
    skill_id: &str,
    archived: bool,
    state: State<'_, SharedState>,
) -> Result<SkillDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let skill_dir = library_root.join(skill_id);
    let skill_md_path = skill_dir.join("SKILL.md");

    if !skill_md_path.is_file() {
        return Err(AppError::new(format!(
            "SKILL.md not found for skill: {}",
            skill_id
        )));
    }

    let content = std::fs::read_to_string(&skill_md_path)
        .map_err(|e| AppError::new(format!("Failed to read SKILL.md: {}", e)))?;

    let updated = set_frontmatter_archived(&content, archived);
    std::fs::write(&skill_md_path, &updated)
        .map_err(|e| AppError::new(format!("Failed to write SKILL.md: {}", e)))?;

    // Re-scan to build the updated detail
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let skill = library_skills
        .iter()
        .find(|s| s.folder_name == skill_id)
        .ok_or_else(|| AppError::new(format!("Skill not found after update: {}", skill_id)))?;

    let linked_locations = scanner::locations_linking_skill(
        skill_id,
        guard.locations(),
        &library_root,
        &library_skills,
        &library_sets,
    );

    let included_in_sets: Vec<SetRef> = library_sets
        .iter()
        .filter(|(_, def)| def.skills.iter().any(|s| s == skill_id))
        .map(|(id, def)| SetRef {
            id: id.clone(),
            name: def.name.clone(),
        })
        .collect();

    let usage = scanner::skill_usage(skill_id, &guard.inner.usage);

    Ok(SkillDetail {
        id: skill.folder_name.clone(),
        name: skill.name.clone(),
        path: skill.path.clone(),
        archived: skill.archived,
        summary: skill.description.clone(),
        linked_locations,
        included_in_sets,
        usage,
    })
}

/// Update or insert the `archived` field in SKILL.md YAML frontmatter.
fn set_frontmatter_archived(content: &str, archived: bool) -> String {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        // No frontmatter — wrap in new frontmatter
        return format!("---\narchived: {}\n---\n{}", archived, content);
    }

    let after_first = &trimmed[3..];
    let end_idx = match after_first.find("---") {
        Some(i) => i,
        None => return content.to_string(),
    };

    let yaml_block = &after_first[..end_idx];
    let rest = &after_first[end_idx..]; // includes the closing ---

    // Check if `archived:` line exists
    let mut found = false;
    let mut new_lines: Vec<String> = Vec::new();
    for line in yaml_block.lines() {
        if line.trim().starts_with("archived:") || line.trim().starts_with("archived :") {
            new_lines.push(format!("archived: {}", archived));
            found = true;
        } else {
            new_lines.push(line.to_string());
        }
    }
    if !found {
        new_lines.push(format!("archived: {}", archived));
    }

    format!("---\n{}{}", new_lines.join("\n"), rest)
}
