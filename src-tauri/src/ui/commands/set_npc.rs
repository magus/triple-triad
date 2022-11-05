use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn set_npc(search: &str, app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    let npc = state.npc_data.search(search).first().unwrap().clone();
    game.computer.cards_from(npc.cards.clone());
    game.rules.from(&npc.rules);

    // assign the game back to mutex
    state.set_game(game);

    // send back the updated game state
    Ok(state.json())
}
