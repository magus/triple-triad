use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn start(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let game = state.game.lock().unwrap().clone();

    // capture the game as-is for resetting
    state.set_setup_game(game);

    // send back the updated game state
    Ok(state.json())
}
