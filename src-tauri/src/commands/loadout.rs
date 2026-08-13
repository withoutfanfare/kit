use tauri::State;

use crate::commands::AppError;
use crate::domain::SessionLoadout;
use crate::resolver;
use crate::state::SharedState;

/// What reaches a session opened at this location, and what it costs.
#[tauri::command]
pub async fn resolve_session_loadout(
    location_id: String,
    state: State<'_, SharedState>,
) -> Result<SessionLoadout, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let location = guard
        .find_location(&location_id)
        .ok_or_else(|| AppError::new(format!("Location not found: {}", location_id)))?
        .clone();
    drop(guard);

    let home = dirs::home_dir()
        .ok_or_else(|| AppError::new("Could not determine the home directory".to_string()))?;

    Ok(resolver::resolve(&location, &home))
}
