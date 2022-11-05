use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    state: AppStateJson,
}

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn state(app_handle: tauri::AppHandle) -> Result<Payload, String> {
    let app_handle_state = app_handle.state::<AppState>();

    Ok(Payload {
        state: app_handle_state.json(),
    })
}
