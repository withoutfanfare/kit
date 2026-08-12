use std::path::{Path, PathBuf};
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;
use crate::usage::UsageIndex;

/// Library-wide usage, read from the hook's logs.
#[tauri::command]
pub fn get_usage_summary(state: State<'_, SharedState>) -> Result<UsageSummary, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    drop(guard);

    let library_root = PathBuf::from(&prefs.library_root);
    let index = UsageIndex::load(&library_root);
    let library_skills = scanner::scan_library_skills(&library_root);

    let mut most_used: Vec<UsageEntry> = Vec::new();
    let mut recently_used: Vec<RecentEntry> = Vec::new();
    let mut unused: Vec<UnusedEntry> = Vec::new();

    for skill in library_skills.iter().filter(|s| !s.archived) {
        let usage = index.for_skill(&skill.folder_name);

        if usage.use_count_30d > 0 {
            most_used.push(UsageEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
                count: usage.use_count_30d,
            });
        }

        match usage.last_used_at {
            Some(_) => recently_used.push(RecentEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
                last_used_at: usage.last_used_at,
            }),
            // Only meaningful when there are logs to be absent from.
            None if index.is_available() => unused.push(UnusedEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
            }),
            None => {}
        }
    }

    most_used.sort_by(|a, b| b.count.cmp(&a.count));
    recently_used.sort_by(|a, b| b.last_used_at.cmp(&a.last_used_at));

    let mut suggestions: Vec<String> = Vec::new();
    if !index.is_available() {
        suggestions.push(format!(
            "No usage logs found under {}. Kit reports usage from the Skill hook's \
             logs, so there is nothing to show yet rather than nothing to report.",
            crate::usage::log_dir(&library_root).display()
        ));
    } else if !unused.is_empty() {
        suggestions.push(format!(
            "{} skills have never been used in {} recorded invocations. Unlinking \
             them from Global keeps them in the library while freeing the context \
             they cost every session.",
            unused.len(),
            index.event_count()
        ));
    }

    Ok(UsageSummary {
        most_used,
        recently_used,
        unused,
        suggestions,
    })
}

/// What is linked into one location, set against what has actually been used
/// there. The gap between the two is the reason this view exists.
#[tauri::command]
pub fn get_location_usage(
    location_id: String,
    state: State<'_, SharedState>,
) -> Result<LocationUsage, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let location = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();
    drop(guard);

    let library_root = PathBuf::from(&prefs.library_root);
    let index = UsageIndex::load(&library_root);
    let library_skills = scanner::scan_library_skills(&library_root);
    let library_sets = scanner::scan_library_sets(&library_root);

    let scan = scanner::scan_location(
        Path::new(&location.path),
        location.kind,
        &library_root,
        &library_skills,
        &library_sets,
    );

    // Global's skills load everywhere, so "used here" would be the whole record;
    // for a project, only invocations inside its directory count.
    let scope: Option<PathBuf> = (!location.is_global()).then(|| PathBuf::from(&location.path));

    let mut rows: Vec<LocationUsageRow> = scan
        .skills
        .iter()
        .map(|assignment| {
            let uses_here = match &scope {
                Some(root) => index.for_skill_in(&assignment.skill_id, root),
                None => index.for_skill(&assignment.skill_id).use_count_30d,
            };
            let overall = index.for_skill(&assignment.skill_id);
            LocationUsageRow {
                skill_id: assignment.skill_id.clone(),
                name: assignment.name.clone(),
                uses_here,
                uses_anywhere: overall.use_count_30d,
                last_used_at: overall.last_used_at,
            }
        })
        .collect();

    rows.sort_by(|a, b| b.uses_here.cmp(&a.uses_here).then_with(|| a.name.cmp(&b.name)));

    Ok(LocationUsage {
        location_id: location.id,
        location_label: location.label,
        available: index.is_available(),
        recorded_since: index.earliest(),
        event_count: index.event_count(),
        linked_count: rows.len(),
        used_here_count: rows.iter().filter(|r| r.uses_here > 0).count(),
        rows,
    })
}
