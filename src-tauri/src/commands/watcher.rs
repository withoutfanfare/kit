use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::state::SharedState;
use crate::watcher::{ManagedWatcher, WatcherStatus};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatcherStatusResponse {
    pub status: WatcherStatus,
    pub watched_path: Option<String>,
}

#[tauri::command]
pub fn start_library_watcher(
    app_handle: tauri::AppHandle,
    state: State<'_, SharedState>,
    watcher: State<'_, ManagedWatcher>,
) -> Result<WatcherStatusResponse, AppError> {
    let library_root = {
        let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
        guard.preferences().library_root.clone()
    };

    if library_root.is_empty() {
        return Err(AppError::new("No library root configured"));
    }

    watcher
        .start(PathBuf::from(&library_root), app_handle)
        .map_err(AppError::new)?;

    Ok(WatcherStatusResponse {
        status: watcher.status(),
        watched_path: watcher.watched_path(),
    })
}

#[tauri::command]
pub fn stop_library_watcher(
    watcher: State<'_, ManagedWatcher>,
) -> Result<WatcherStatusResponse, AppError> {
    watcher.stop().map_err(AppError::new)?;
    Ok(WatcherStatusResponse {
        status: watcher.status(),
        watched_path: None,
    })
}

#[tauri::command]
pub fn get_watcher_status(
    watcher: State<'_, ManagedWatcher>,
) -> Result<WatcherStatusResponse, AppError> {
    Ok(WatcherStatusResponse {
        status: watcher.status(),
        watched_path: watcher.watched_path(),
    })
}
