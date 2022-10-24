use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    state: AppStateJson,
    app_handle_state: AppStateJson,
}

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn greet(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    name: &str,
) -> Result<String, String> {
    let app_handle_state = app_handle.state::<AppState>();

    let payload = Payload {
        message: "Place NPC card on board".into(),
        state: state.json(),
        app_handle_state: app_handle_state.json(),
    };

    app_handle.emit_all("select", payload).unwrap();

    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
