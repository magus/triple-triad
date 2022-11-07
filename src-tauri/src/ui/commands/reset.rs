use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn reset(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let setup_game = state.setup_game.lock().unwrap().clone();

    // assign the game back to mutex
    state.set_game(setup_game);
    state.set_explore_result(None);

    // send back the updated game state
    Ok(state.json())
}
