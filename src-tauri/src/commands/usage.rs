use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn get_usage_summary(state: State<'_, SharedState>) -> Result<UsageSummary, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);

    let mut most_used: Vec<UsageEntry> = Vec::new();
    let mut recently_used: Vec<RecentEntry> = Vec::new();
    let mut unused: Vec<UnusedEntry> = Vec::new();

    for skill in &library_skills {
        if skill.archived {
            continue;
        }

        let usage = scanner::skill_usage(&skill.folder_name, &guard.inner.usage);

        if usage.use_count_30d > 0 {
            most_used.push(UsageEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
                count: usage.use_count_30d,
            });
        }

        if usage.last_used_at.is_some() {
            recently_used.push(RecentEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
                last_used_at: usage.last_used_at,
            });
        } else {
            unused.push(UnusedEntry {
                id: skill.folder_name.clone(),
                name: skill.name.clone(),
            });
        }
    }

    // Sort most used by count descending
    most_used.sort_by(|a, b| b.count.cmp(&a.count));

    // Sort recently used by date descending
    recently_used.sort_by(|a, b| b.last_used_at.cmp(&a.last_used_at));

    // Generate suggestions
    let mut suggestions: Vec<String> = Vec::new();
    if !unused.is_empty() {
        suggestions.push(format!(
            "You have {} skills that have never been used. Consider archiving or removing them.",
            unused.len()
        ));
    }
    if most_used.len() > 3 {
        suggestions.push(
            "Your most-used skills could be grouped into a set for quick assignment.".to_string(),
        );
    }

    Ok(UsageSummary {
        most_used,
        recently_used,
        unused,
        suggestions,
    })
}
