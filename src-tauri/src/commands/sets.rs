use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a human-readable name into a kebab-case ID suitable for filenames.
fn to_kebab_case(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        // Collapse runs of dashes
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Resolve the directory and file path for a set given its scope.
fn resolve_set_path(
    set_id: &str,
    scope: &SetScope,
    owner_location_id: Option<&str>,
    library_root: &PathBuf,
    locations: &[SavedLocation],
) -> Result<PathBuf, AppError> {
    match scope {
        SetScope::Global => {
            let sets_dir = library_root.join("sets");
            Ok(sets_dir.join(format!("{}.set.json", set_id)))
        }
        SetScope::Project => {
            let loc_id = owner_location_id
                .ok_or_else(|| AppError::new("owner_location_id required for project scope"))?;
            let loc = locations
                .iter()
                .find(|l| l.id == loc_id)
                .ok_or_else(|| AppError::new(format!("Location not found: {}", loc_id)))?;
            let sets_dir = PathBuf::from(&loc.path).join(".claude").join("sets");
            Ok(sets_dir.join(format!("{}.set.json", set_id)))
        }
    }
}

/// Parse a scope string into SetScope.
fn parse_scope(scope: &str) -> Result<SetScope, AppError> {
    match scope {
        "global" => Ok(SetScope::Global),
        "project" => Ok(SetScope::Project),
        other => Err(AppError::new(format!(
            "Invalid scope '{}': expected 'global' or 'project'",
            other
        ))),
    }
}

/// Build skill entries for a set definition, resolving names and archived
/// status from the library.
fn build_skill_entries(
    skill_ids: &[String],
    library_skills: &[SkillMeta],
) -> Vec<SetSkillEntry> {
    skill_ids
        .iter()
        .map(|sid| {
            let meta = library_skills.iter().find(|s| s.folder_name == *sid);
            SetSkillEntry {
                id: sid.clone(),
                name: meta
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| sid.clone()),
                archived: meta.map(|m| m.archived).unwrap_or(false),
            }
        })
        .collect()
}

/// Count how many saved locations have this set assigned in their manifest.
fn count_assigned_locations(
    set_id: &str,
    locations: &[SavedLocation],
) -> usize {
    locations
        .iter()
        .filter(|loc| {
            let manifest_path = PathBuf::from(&loc.path)
                .join(".claude")
                .join("settings.json");
            let manifest_sets = scanner::read_manifest_sets(&manifest_path);
            manifest_sets.contains(&set_id.to_string())
        })
        .count()
}

/// Build the list of locations that have this set assigned.
fn build_assigned_locations(
    set_id: &str,
    locations: &[SavedLocation],
    library_root: &PathBuf,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Vec<SavedLocationSummary> {
    locations
        .iter()
        .filter(|loc| {
            let manifest_path = PathBuf::from(&loc.path)
                .join(".claude")
                .join("settings.json");
            let manifest_sets = scanner::read_manifest_sets(&manifest_path);
            manifest_sets.contains(&set_id.to_string())
        })
        .map(|loc| {
            scanner::build_location_summary(loc, library_root, library_skills, library_sets)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn list_sets(
    state: State<'_, SharedState>,
) -> Result<Vec<SetSummary>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();

    let mut summaries: Vec<SetSummary> = Vec::new();

    // Global sets from <libraryRoot>/sets/
    let global_sets = scanner::scan_library_sets(&library_root);
    for (set_id, set_def) in &global_sets {
        let set_path = library_root
            .join("sets")
            .join(format!("{}.set.json", set_id))
            .to_string_lossy()
            .to_string();
        summaries.push(SetSummary {
            id: set_id.clone(),
            name: set_def.name.clone(),
            description: set_def.description.clone(),
            scope: SetScope::Global,
            owner_location_id: None,
            skill_count: set_def.skills.len(),
            assigned_location_count: count_assigned_locations(set_id, &locations),
            path: set_path,
        });
    }

    // Project sets from each location's .claude/sets/
    for loc in &locations {
        let loc_path = PathBuf::from(&loc.path);
        let project_sets = scanner::scan_project_sets(&loc_path);
        for (set_id, set_def) in &project_sets {
            let set_path = loc_path
                .join(".claude")
                .join("sets")
                .join(format!("{}.set.json", set_id))
                .to_string_lossy()
                .to_string();
            summaries.push(SetSummary {
                id: set_id.clone(),
                name: set_def.name.clone(),
                description: set_def.description.clone(),
                scope: SetScope::Project,
                owner_location_id: Some(loc.id.clone()),
                skill_count: set_def.skills.len(),
                assigned_location_count: count_assigned_locations(set_id, &locations),
                path: set_path,
            });
        }
    }

    Ok(summaries)
}

#[tauri::command]
pub fn create_set(
    name: String,
    scope: String,
    owner_location_id: Option<String>,
    description: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SetDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_id = to_kebab_case(&name);
    if set_id.is_empty() {
        return Err(AppError::new("Set name cannot be empty"));
    }

    // Ensure the target directory exists
    let sets_dir = match parsed_scope {
        SetScope::Global => {
            scanner::ensure_global_sets_dir(&library_root).map_err(AppError::new)?
        }
        SetScope::Project => {
            let loc_id = owner_location_id
                .as_deref()
                .ok_or_else(|| AppError::new("owner_location_id required for project scope"))?;
            let loc = locations
                .iter()
                .find(|l| l.id == loc_id)
                .ok_or_else(|| AppError::new(format!("Location not found: {}", loc_id)))?;
            scanner::ensure_project_sets_dir(&PathBuf::from(&loc.path))
                .map_err(AppError::new)?
        }
    };

    let set_file = sets_dir.join(format!("{}.set.json", set_id));
    if set_file.exists() {
        return Err(AppError::new(format!(
            "Set already exists: {}",
            set_id
        )));
    }

    let def = SetDefinition {
        name: name.clone(),
        description: description.clone(),
        skills: Vec::new(),
    };

    let json = serde_json::to_string_pretty(&def)?;
    std::fs::write(&set_file, json)?;

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    Ok(SetDetail {
        id: set_id,
        name,
        description,
        scope: parsed_scope,
        owner_location_id,
        path: set_file.to_string_lossy().to_string(),
        skills: Vec::new(),
        assigned_locations: build_assigned_locations(
            &def.name,
            &locations,
            &library_root,
            &library_skills,
            &library_sets,
        ),
    })
}

#[tauri::command]
pub fn get_set_detail(
    set_id: String,
    scope: String,
    owner_location_id: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SetDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_path = resolve_set_path(
        &set_id,
        &parsed_scope,
        owner_location_id.as_deref(),
        &library_root,
        &locations,
    )?;

    if !set_path.is_file() {
        return Err(AppError::new(format!("Set file not found: {}", set_id)));
    }

    let content = std::fs::read_to_string(&set_path)?;
    let def: SetDefinition = serde_json::from_str(&content)?;

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let skill_entries = build_skill_entries(&def.skills, &library_skills);
    let assigned_locations = build_assigned_locations(
        &set_id,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    );

    Ok(SetDetail {
        id: set_id,
        name: def.name,
        description: def.description,
        scope: parsed_scope,
        owner_location_id,
        path: set_path.to_string_lossy().to_string(),
        skills: skill_entries,
        assigned_locations,
    })
}

#[tauri::command]
pub fn update_set(
    set_id: String,
    scope: String,
    owner_location_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    skill_ids: Option<Vec<String>>,
    state: State<'_, SharedState>,
) -> Result<SetDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_path = resolve_set_path(
        &set_id,
        &parsed_scope,
        owner_location_id.as_deref(),
        &library_root,
        &locations,
    )?;

    if !set_path.is_file() {
        return Err(AppError::new(format!("Set file not found: {}", set_id)));
    }

    let content = std::fs::read_to_string(&set_path)?;
    let mut def: SetDefinition = serde_json::from_str(&content)?;

    if let Some(n) = name {
        def.name = n;
    }
    if let Some(d) = description {
        def.description = Some(d);
    }
    if let Some(sids) = skill_ids {
        def.skills = sids;
    }

    let json = serde_json::to_string_pretty(&def)?;
    std::fs::write(&set_path, json)?;

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let skill_entries = build_skill_entries(&def.skills, &library_skills);
    let assigned_locations = build_assigned_locations(
        &set_id,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    );

    Ok(SetDetail {
        id: set_id,
        name: def.name,
        description: def.description,
        scope: parsed_scope,
        owner_location_id,
        path: set_path.to_string_lossy().to_string(),
        skills: skill_entries,
        assigned_locations,
    })
}

#[tauri::command]
pub fn add_skill_to_set(
    set_id: String,
    skill_id: String,
    scope: String,
    owner_location_id: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SetDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_path = resolve_set_path(
        &set_id,
        &parsed_scope,
        owner_location_id.as_deref(),
        &library_root,
        &locations,
    )?;

    if !set_path.is_file() {
        return Err(AppError::new(format!("Set file not found: {}", set_id)));
    }

    let content = std::fs::read_to_string(&set_path)?;
    let mut def: SetDefinition = serde_json::from_str(&content)?;

    if !def.skills.contains(&skill_id) {
        def.skills.push(skill_id);
    }

    let json = serde_json::to_string_pretty(&def)?;
    std::fs::write(&set_path, json)?;

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let skill_entries = build_skill_entries(&def.skills, &library_skills);
    let assigned_locations = build_assigned_locations(
        &set_id,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    );

    Ok(SetDetail {
        id: set_id,
        name: def.name,
        description: def.description,
        scope: parsed_scope,
        owner_location_id,
        path: set_path.to_string_lossy().to_string(),
        skills: skill_entries,
        assigned_locations,
    })
}

#[tauri::command]
pub fn remove_skill_from_set(
    set_id: String,
    skill_id: String,
    scope: String,
    owner_location_id: Option<String>,
    state: State<'_, SharedState>,
) -> Result<SetDetail, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_path = resolve_set_path(
        &set_id,
        &parsed_scope,
        owner_location_id.as_deref(),
        &library_root,
        &locations,
    )?;

    if !set_path.is_file() {
        return Err(AppError::new(format!("Set file not found: {}", set_id)));
    }

    let content = std::fs::read_to_string(&set_path)?;
    let mut def: SetDefinition = serde_json::from_str(&content)?;

    def.skills.retain(|s| s != &skill_id);

    let json = serde_json::to_string_pretty(&def)?;
    std::fs::write(&set_path, json)?;

    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);
    let skill_entries = build_skill_entries(&def.skills, &library_skills);
    let assigned_locations = build_assigned_locations(
        &set_id,
        &locations,
        &library_root,
        &library_skills,
        &library_sets,
    );

    Ok(SetDetail {
        id: set_id,
        name: def.name,
        description: def.description,
        scope: parsed_scope,
        owner_location_id,
        path: set_path.to_string_lossy().to_string(),
        skills: skill_entries,
        assigned_locations,
    })
}

#[tauri::command]
pub fn delete_set(
    set_id: String,
    scope: String,
    owner_location_id: Option<String>,
    state: State<'_, SharedState>,
) -> Result<Vec<SetSummary>, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();
    let parsed_scope = parse_scope(&scope)?;

    let set_path = resolve_set_path(
        &set_id,
        &parsed_scope,
        owner_location_id.as_deref(),
        &library_root,
        &locations,
    )?;

    if !set_path.is_file() {
        return Err(AppError::new(format!("Set file not found: {}", set_id)));
    }

    std::fs::remove_file(&set_path)?;

    // Drop the guard and re-acquire to call list_sets logic
    drop(guard);

    // Rebuild the full list
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let locations = guard.locations().to_vec();

    let mut summaries: Vec<SetSummary> = Vec::new();

    let global_sets = scanner::scan_library_sets(&library_root);
    for (sid, sdef) in &global_sets {
        let sp = library_root
            .join("sets")
            .join(format!("{}.set.json", sid))
            .to_string_lossy()
            .to_string();
        summaries.push(SetSummary {
            id: sid.clone(),
            name: sdef.name.clone(),
            description: sdef.description.clone(),
            scope: SetScope::Global,
            owner_location_id: None,
            skill_count: sdef.skills.len(),
            assigned_location_count: count_assigned_locations(sid, &locations),
            path: sp,
        });
    }

    for loc in &locations {
        let loc_path = PathBuf::from(&loc.path);
        let project_sets = scanner::scan_project_sets(&loc_path);
        for (sid, sdef) in &project_sets {
            let sp = loc_path
                .join(".claude")
                .join("sets")
                .join(format!("{}.set.json", sid))
                .to_string_lossy()
                .to_string();
            summaries.push(SetSummary {
                id: sid.clone(),
                name: sdef.name.clone(),
                description: sdef.description.clone(),
                scope: SetScope::Project,
                owner_location_id: Some(loc.id.clone()),
                skill_count: sdef.skills.len(),
                assigned_location_count: count_assigned_locations(sid, &locations),
                path: sp,
            });
        }
    }

    Ok(summaries)
}
