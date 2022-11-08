use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn status(status: String, app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    return status_internal(status, &app_handle);
}

pub fn status_internal(
    status: String,
    app_handle: &tauri::AppHandle,
) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    state.set_status(Some(status));

    // send back the updated game state
    Ok(state.json())
}
