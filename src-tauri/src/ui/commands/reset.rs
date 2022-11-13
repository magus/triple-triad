use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use super::pre_game::pre_game_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn reset(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    return reset_internal(&app_handle);
}

pub fn reset_internal(app_handle: &tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let setup_game = state.setup_game.lock().unwrap().clone();

    state.set_game(setup_game);
    state.set_explore_result(None);
    state.set_status(None);
    state.set_pre_game(None);

    // handle pre_game setup immediately
    pre_game_internal(&app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
