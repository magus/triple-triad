use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use super::start::start_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn set_npc(search: &str, app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    let npc_data_ref = state.npc_data.lock();
    let npc_data = npc_data_ref.as_ref().unwrap().as_ref().unwrap();

    let npc = npc_data.search(search).first().unwrap().clone();

    // capture selected npc
    state.set_npc(Some(npc.clone()));

    game.computer.cards_from(npc.cards.clone());
    game.rules.from(&npc.rules);

    // assign the game back to mutex
    state.set_game(game);

    // handle saving setup_game and reset
    start_internal(&app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
