use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn chaos_select(
    card: usize,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    game.chaos_card = Some(card);

    // assign the game back to mutex
    state.set_game(game);

    // reset chaos_select status
    state.set_status(None);

    // send back the updated game state
    Ok(state.json())
}
