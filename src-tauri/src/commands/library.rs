use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::{self, SharedState};

#[tauri::command]
pub fn list_library_items(
    state: State<'_, SharedState>,
) -> Result<Vec<LibraryListItem>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    // Scan each location once and cache results
    let location_scans: Vec<scanner::LocationScanResult> = guard
        .locations()
        .iter()
        .map(|loc| {
            let loc_path = PathBuf::from(&loc.path);
            scanner::scan_location(&loc_path, &library_root, &library_skills, &library_sets)
        })
        .collect();

    let mut items: Vec<LibraryListItem> = Vec::new();

    // Add skills — look up linked count from cached scans, plus usage data
    for skill in &library_skills {
        let linked_count = location_scans
            .iter()
            .filter(|scan| {
                scan.skills
                    .iter()
                    .any(|s| s.skill_id == skill.folder_name && s.link_state == LinkState::Linked)
            })
            .count();

        let usage = scanner::skill_usage(&skill.folder_name, &guard.inner.usage);

        items.push(LibraryListItem {
            id: skill.folder_name.clone(),
            name: skill.name.clone(),
            kind: LibraryItemKind::Skill,
            archived: skill.archived,
            summary: skill.description.clone(),
            linked_location_count: linked_count,
            use_count_30d: usage.use_count_30d,
            last_used_at: usage.last_used_at,
            is_unused_everywhere: linked_count == 0,
        });
    }

    // Add sets — check if any set skill is linked at each location
    for (set_id, set_def) in &library_sets {
        let linked_count = location_scans
            .iter()
            .filter(|scan| {
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
            use_count_30d: 0,
            last_used_at: None,
            is_unused_everywhere: linked_count == 0,
        });
    }

    Ok(items)
}

#[tauri::command]
pub fn get_skill_detail(
    skill_id: String,
    skill_path: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SkillDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    // Try library first
    if let Some(skill) = library_skills.iter().find(|s| s.folder_name == skill_id) {
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

        return Ok(SkillDetail {
            id: skill.folder_name.clone(),
            name: skill.name.clone(),
            path: skill.path.clone(),
            archived: skill.archived,
            summary: skill.description.clone(),
            linked_locations,
            included_in_sets,
            usage,
        });
    }

    // Fall back to reading SKILL.md from the provided path (for local-only skills)
    if let Some(ref path_str) = skill_path {
        let skill_dir = PathBuf::from(path_str);
        let skill_md = skill_dir.join("SKILL.md");
        if skill_md.is_file() {
            if let Ok(content) = std::fs::read_to_string(&skill_md) {
                if let Some(fm) = scanner::parse_skill_md(&content) {
                    return Ok(SkillDetail {
                        id: skill_id,
                        name: fm.name,
                        path: path_str.clone(),
                        archived: fm.archived,
                        summary: fm.description,
                        linked_locations: Vec::new(),
                        included_in_sets: Vec::new(),
                        usage: SkillUsage {
                            last_used_at: None,
                            use_count_30d: 0,
                        },
                    });
                }
            }
        }
    }

    Err(AppError::new(format!("Skill not found: {}", skill_id)))
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
    state::atomic_write(&skill_md_path, &updated)
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
    // Find the frontmatter boundaries in the original content (preserving leading bytes)
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        // No frontmatter — wrap in new frontmatter
        return format!("---\narchived: {}\n---\n{}", archived, content);
    }

    // Calculate the byte offset of the frontmatter start in the original content
    let leading_len = content.len() - trimmed.len();
    let leading = &content[..leading_len];

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

    format!("{}---\n{}\n{}", leading, new_lines.join("\n"), rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_archived_updates_existing() {
        let content = "---\nname: Test\narchived: false\n---\nBody";
        let result = set_frontmatter_archived(content, true);
        assert!(result.contains("archived: true"));
        assert!(!result.contains("archived: false"));
    }

    #[test]
    fn set_archived_inserts_when_missing() {
        let content = "---\nname: Test\n---\nBody";
        let result = set_frontmatter_archived(content, true);
        assert!(result.contains("archived: true"));
        assert!(result.contains("name: Test"));
    }

    #[test]
    fn set_archived_no_frontmatter() {
        let content = "Just body text";
        let result = set_frontmatter_archived(content, true);
        assert!(result.contains("---\narchived: true\n---"));
        assert!(result.contains("Just body text"));
    }

    #[test]
    fn set_archived_false() {
        let content = "---\nname: Test\narchived: true\n---\n";
        let result = set_frontmatter_archived(content, false);
        assert!(result.contains("archived: false"));
        assert!(!result.contains("archived: true"));
    }
}
