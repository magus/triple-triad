use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn explore(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let game = state.game.lock().unwrap().clone();

    if game.turn_is_player() {
        let explore_result = game.evaluate_explore_result();
        state.set_explore_result(Some(explore_result));
    }

    // send back the updated game state
    Ok(state.json())
}
