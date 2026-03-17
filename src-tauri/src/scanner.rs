use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::*;
use crate::state::UsageRecord;

// ---------------------------------------------------------------------------
// SKILL.md frontmatter parsing
// ---------------------------------------------------------------------------

/// Parse YAML frontmatter from a SKILL.md file.
/// Expects the file to start with `---`, then YAML, then `---`.
pub fn parse_skill_md(content: &str) -> Option<SkillFrontmatter> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let after_first = &trimmed[3..];
    let end = after_first.find("---")?;
    let yaml_block = &after_first[..end];

    // Minimal hand-parser for YAML key: value lines so we avoid pulling in a
    // full YAML crate.  We only care about a handful of top-level scalars.
    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    let mut version: Option<String> = None;
    let mut archived = false;

    for line in yaml_block.lines() {
        let line = line.trim();
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim().trim_matches('"').trim_matches('\'');
            match key {
                "name" => name = Some(val.to_string()),
                "description" => description = Some(val.to_string()),
                "version" => version = Some(val.to_string()),
                "archived" => archived = val == "true",
                _ => {}
            }
        }
    }

    Some(SkillFrontmatter {
        name: name?,
        description,
        version,
        archived,
    })
}

#[derive(Debug, Clone)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub archived: bool,
}

// ---------------------------------------------------------------------------
// Library scanning
// ---------------------------------------------------------------------------

/// Scan the library root for skills (folders containing SKILL.md).
pub fn scan_library_skills(library_root: &Path) -> Vec<SkillMeta> {
    let mut skills = Vec::new();
    if !library_root.is_dir() {
        return skills;
    }

    let entries = match fs::read_dir(library_root) {
        Ok(e) => e,
        Err(_) => return skills,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        // Skip hidden/system directories
        let folder_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if folder_name.starts_with('.') {
            continue;
        }

        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let content = match fs::read_to_string(&skill_md) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some(fm) = parse_skill_md(&content) {
            let canonical_path = fs::canonicalize(&path)
                .ok()
                .map(|p| p.to_string_lossy().to_string());
            skills.push(SkillMeta {
                name: fm.name,
                description: fm.description,
                version: fm.version,
                archived: fm.archived,
                folder_name: folder_name.clone(),
                path: path.to_string_lossy().to_string(),
                canonical_path,
            });
        }
    }

    skills
}

/// Scan library root for set definition files in `<libraryRoot>/sets/*.set.json`.
pub fn scan_library_sets(library_root: &Path) -> Vec<(String, SetDefinition)> {
    let sets_dir = library_root.join("sets");
    scan_sets_in_dir(&sets_dir)
}

/// Scan a project location for set definition files in `<location>/.claude/sets/*.set.json`.
pub fn scan_project_sets(location_path: &Path) -> Vec<(String, SetDefinition)> {
    let sets_dir = location_path.join(".claude").join("sets");
    scan_sets_in_dir(&sets_dir)
}

/// Shared helper: scan a directory for `*.set.json` files.
fn scan_sets_in_dir(dir: &Path) -> Vec<(String, SetDefinition)> {
    let mut sets = Vec::new();
    if !dir.is_dir() {
        return sets;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return sets,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if file_name.ends_with(".set.json") && path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(def) = serde_json::from_str::<SetDefinition>(&content) {
                    let id = file_name.trim_end_matches(".set.json").to_string();
                    sets.push((id, def));
                }
            }
        }
    }

    sets
}

/// Ensure the global sets directory exists at `<libraryRoot>/sets/`.
pub fn ensure_global_sets_dir(library_root: &Path) -> Result<PathBuf, String> {
    let sets_dir = library_root.join("sets");
    fs::create_dir_all(&sets_dir).map_err(|e| {
        format!(
            "Failed to create global sets directory {}: {}",
            sets_dir.display(),
            e
        )
    })?;
    Ok(sets_dir)
}

/// Ensure the project sets directory exists at `<location>/.claude/sets/`.
pub fn ensure_project_sets_dir(location_path: &Path) -> Result<PathBuf, String> {
    let sets_dir = location_path.join(".claude").join("sets");
    fs::create_dir_all(&sets_dir).map_err(|e| {
        format!(
            "Failed to create project sets directory {}: {}",
            sets_dir.display(),
            e
        )
    })?;
    Ok(sets_dir)
}

/// Read declared set IDs from the manifest's `sets` array.
pub fn read_manifest_sets(manifest_path: &Path) -> Vec<String> {
    let content = match fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    match value.get("sets") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Location scanning
// ---------------------------------------------------------------------------

/// Discover the skills directory inside a project location.
/// Checks `.claude/skills/` first, then `skills/`.
pub fn find_skills_dir(location_path: &Path) -> Option<PathBuf> {
    let claude_skills = location_path.join(".claude").join("skills");
    if claude_skills.is_dir() {
        return Some(claude_skills);
    }
    let plain_skills = location_path.join("skills");
    if plain_skills.is_dir() {
        return Some(plain_skills);
    }
    None
}

/// Discover the manifest file (`.claude/settings.json` or similar).
pub fn find_manifest_path(location_path: &Path) -> Option<PathBuf> {
    let settings = location_path.join(".claude").join("settings.json");
    if settings.is_file() {
        return Some(settings);
    }
    None
}

/// Read declared skill names from the manifest's `skills` array.
pub fn read_manifest_skills(manifest_path: &Path) -> Vec<String> {
    let content = match fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let value: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    match value.get("skills") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Full scan of a location, producing skill assignments, set assignments, and
/// issues, given a set of known library skills.
pub fn scan_location(
    location_path: &Path,
    library_root: &Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> LocationScanResult {
    let mut skill_assignments: Vec<SkillAssignment> = Vec::new();
    let mut set_assignments: Vec<SetAssignment> = Vec::new();
    let mut issues: Vec<LocationIssue> = Vec::new();

    let lib_skill_map: HashMap<&str, &SkillMeta> = library_skills
        .iter()
        .map(|s| (s.folder_name.as_str(), s))
        .collect();

    // Build a set of canonical library skill paths for symlink target matching.
    // This handles the case where the library root contains symlinks itself
    // (e.g., ~/.claude/skills/X -> /path/to/real/skills/X).
    let canonical_lib_paths: HashMap<String, &str> = library_skills
        .iter()
        .filter_map(|s| {
            s.canonical_path
                .as_ref()
                .map(|cp| (cp.clone(), s.folder_name.as_str()))
        })
        .collect();

    let skills_dir = find_skills_dir(location_path);
    let manifest_path = find_manifest_path(location_path);
    let manifest_skills = manifest_path
        .as_ref()
        .map(|p| read_manifest_skills(p))
        .unwrap_or_default();

    // Track which manifest entries we've seen
    let mut seen_manifest_entries: Vec<String> = Vec::new();

    // Scan files in the skills directory
    if let Some(ref sd) = skills_dir {
        if let Ok(entries) = fs::read_dir(sd) {
            for entry in entries.flatten() {
                let path = entry.path();
                let entry_name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n.to_string(),
                    None => continue,
                };
                if entry_name.starts_with('.') {
                    continue;
                }

                let symlink_meta = fs::symlink_metadata(&path);
                let is_symlink = symlink_meta
                    .as_ref()
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);

                let declared = manifest_skills.contains(&entry_name);
                if declared {
                    seen_manifest_entries.push(entry_name.clone());
                }

                if is_symlink {
                    // Resolve the symlink target
                    let target = fs::read_link(&path);
                    let target_exists = path.exists(); // follows symlink

                    if !target_exists {
                        // Broken link
                        skill_assignments.push(SkillAssignment {
                            skill_id: entry_name.clone(),
                            name: entry_name.clone(),
                            path: path.to_string_lossy().to_string(),
                            link_state: LinkState::BrokenLink,
                            declared_in_manifest: declared,
                            archived: false,
                            source: SkillSource::Library,
                        });
                        issues.push(LocationIssue {
                            kind: IssueKind::BrokenLink,
                            skill_name: entry_name.clone(),
                            skill_id: Some(entry_name.clone()),
                            message: format!(
                                "Symlink for '{}' points to a non-existent target",
                                entry_name
                            ),
                        });
                    } else {
                        // Valid symlink — check if it points into the library
                        // by comparing canonical paths (handles symlinked library roots)
                        let target_path = target.ok();
                        let canon_target = target_path
                            .as_ref()
                            .and_then(|t| {
                                let resolved = if t.is_relative() {
                                    sd.join(t)
                                } else {
                                    t.clone()
                                };
                                fs::canonicalize(&resolved).ok()
                            });

                        let in_library = canon_target
                            .as_ref()
                            .map(|canon| {
                                let canon_str = canon.to_string_lossy().to_string();
                                // Check by canonical path set (handles symlinked library entries)
                                canonical_lib_paths.contains_key(&canon_str)
                                    // Also check starts_with on canonical library root
                                    || fs::canonicalize(library_root)
                                        .map(|lr| canon_str.starts_with(&lr.to_string_lossy().to_string()))
                                        .unwrap_or(false)
                            })
                            .unwrap_or(false);

                        let (source, archived) = if in_library {
                            let archived = lib_skill_map
                                .get(entry_name.as_str())
                                .map(|m| m.archived)
                                .unwrap_or(false);
                            (SkillSource::Library, archived)
                        } else {
                            (SkillSource::Local, false)
                        };

                        let link_state = if in_library {
                            LinkState::Linked
                        } else {
                            LinkState::LocalOnly
                        };

                        let name = lib_skill_map
                            .get(entry_name.as_str())
                            .map(|m| m.name.clone())
                            .unwrap_or_else(|| entry_name.clone());

                        if !declared && in_library {
                            issues.push(LocationIssue {
                                kind: IssueKind::LinkedUndeclared,
                                skill_name: entry_name.clone(),
                                skill_id: Some(entry_name.clone()),
                                message: format!(
                                    "'{}' is linked but not declared in the manifest",
                                    entry_name
                                ),
                            });
                        }

                        skill_assignments.push(SkillAssignment {
                            skill_id: entry_name.clone(),
                            name,
                            path: path.to_string_lossy().to_string(),
                            link_state,
                            declared_in_manifest: declared,
                            archived,
                            source,
                        });
                    }
                } else if path.is_dir() {
                    // Local directory (not a symlink)
                    let name = lib_skill_map
                        .get(entry_name.as_str())
                        .map(|m| m.name.clone())
                        .unwrap_or_else(|| entry_name.clone());

                    skill_assignments.push(SkillAssignment {
                        skill_id: entry_name.clone(),
                        name,
                        path: path.to_string_lossy().to_string(),
                        link_state: LinkState::LocalOnly,
                        declared_in_manifest: declared,
                        archived: false,
                        source: SkillSource::Local,
                    });
                }
            }
        }
    }

    // Check for manifest entries that have no corresponding file
    for manifest_skill in &manifest_skills {
        if !seen_manifest_entries.contains(manifest_skill) {
            skill_assignments.push(SkillAssignment {
                skill_id: manifest_skill.clone(),
                name: lib_skill_map
                    .get(manifest_skill.as_str())
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| manifest_skill.clone()),
                path: String::new(),
                link_state: LinkState::DeclaredOnly,
                declared_in_manifest: true,
                archived: lib_skill_map
                    .get(manifest_skill.as_str())
                    .map(|m| m.archived)
                    .unwrap_or(false),
                source: SkillSource::Library,
            });
            issues.push(LocationIssue {
                kind: IssueKind::DeclaredMissing,
                skill_name: manifest_skill.clone(),
                skill_id: Some(manifest_skill.clone()),
                message: format!(
                    "'{}' is declared in the manifest but has no matching skill directory",
                    manifest_skill
                ),
            });
        }
    }

    // Set assignments — check which sets are declared in the manifest
    let manifest_set_ids = manifest_path
        .as_ref()
        .map(|p| read_manifest_sets(p))
        .unwrap_or_default();

    for (set_id, set_def) in library_sets {
        if manifest_set_ids.contains(set_id) {
            let set_path = library_root
                .join("sets")
                .join(format!("{}.set.json", set_id))
                .to_string_lossy()
                .to_string();
            set_assignments.push(SetAssignment {
                set_id: set_id.clone(),
                name: set_def.name.clone(),
                skill_count: set_def.skills.len(),
                path: set_path,
            });
        }
    }

    // Also check project sets
    let project_sets = scan_project_sets(location_path);
    for (set_id, set_def) in &project_sets {
        if manifest_set_ids.contains(set_id) {
            let set_path = location_path
                .join(".claude")
                .join("sets")
                .join(format!("{}.set.json", set_id))
                .to_string_lossy()
                .to_string();
            // Avoid duplicates if a global set has the same ID
            if !set_assignments.iter().any(|s| s.set_id == *set_id) {
                set_assignments.push(SetAssignment {
                    set_id: set_id.clone(),
                    name: set_def.name.clone(),
                    skill_count: set_def.skills.len(),
                    path: set_path,
                });
            }
        }
    }

    let stats = LocationStats {
        linked_count: skill_assignments
            .iter()
            .filter(|s| s.link_state == LinkState::Linked)
            .count(),
        local_only_count: skill_assignments
            .iter()
            .filter(|s| s.link_state == LinkState::LocalOnly)
            .count(),
        broken_count: skill_assignments
            .iter()
            .filter(|s| s.link_state == LinkState::BrokenLink)
            .count(),
    };

    LocationScanResult {
        skills: skill_assignments,
        sets: set_assignments,
        issues,
        stats,
        manifest_path: manifest_path.map(|p| p.to_string_lossy().to_string()),
    }
}

pub struct LocationScanResult {
    pub skills: Vec<SkillAssignment>,
    pub sets: Vec<SetAssignment>,
    pub issues: Vec<LocationIssue>,
    pub stats: LocationStats,
    pub manifest_path: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers to build summary from scan results
// ---------------------------------------------------------------------------

/// Count broken links across all locations.
pub fn count_broken_links_for_locations(
    locations: &[crate::domain::SavedLocation],
    library_root: &Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> usize {
    locations
        .iter()
        .map(|loc| {
            let loc_path = PathBuf::from(&loc.path);
            let result = scan_location(&loc_path, library_root, library_skills, library_sets);
            result.stats.broken_count
        })
        .sum()
}

/// Build a SavedLocationSummary for a given SavedLocation by scanning the
/// filesystem.
pub fn build_location_summary(
    loc: &crate::domain::SavedLocation,
    library_root: &Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> SavedLocationSummary {
    let loc_path = PathBuf::from(&loc.path);
    let result = scan_location(&loc_path, library_root, library_skills, library_sets);
    SavedLocationSummary {
        id: loc.id.clone(),
        label: loc.label.clone(),
        path: loc.path.clone(),
        issue_count: result.issues.len(),
        installed_skill_count: result
            .skills
            .iter()
            .filter(|s| s.link_state == LinkState::Linked || s.link_state == LinkState::LocalOnly)
            .count(),
        installed_set_count: result.sets.len(),
        last_synced_at: loc.last_synced_at,
    }
}

/// Determine which locations link to a given skill (by folder name).
pub fn locations_linking_skill(
    skill_folder: &str,
    locations: &[crate::domain::SavedLocation],
    library_root: &Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Vec<SavedLocationSummary> {
    locations
        .iter()
        .filter_map(|loc| {
            let loc_path = PathBuf::from(&loc.path);
            let result = scan_location(&loc_path, library_root, library_skills, library_sets);
            let links = result
                .skills
                .iter()
                .any(|s| s.skill_id == skill_folder && s.link_state == LinkState::Linked);
            if links {
                Some(build_location_summary(
                    loc,
                    library_root,
                    library_skills,
                    library_sets,
                ))
            } else {
                None
            }
        })
        .collect()
}

/// Get usage data for a skill, falling back to defaults.
pub fn skill_usage(
    skill_folder: &str,
    usage_map: &HashMap<String, UsageRecord>,
) -> SkillUsage {
    match usage_map.get(skill_folder) {
        Some(rec) => SkillUsage {
            last_used_at: rec.last_used_at,
            use_count_30d: rec.use_count_30d,
        },
        None => SkillUsage {
            last_used_at: None,
            use_count_30d: 0,
        },
    }
}
