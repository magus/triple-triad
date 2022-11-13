use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use super::pre_game::pre_game_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn swap(
    player: usize,
    computer: usize,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    // println!("[command::all_open] card={:#?}", cards);

    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    game.execute_swap(player, computer);

    state.set_game(game);

    let mut pre_game = state.pre_game.lock().unwrap().clone().unwrap();
    pre_game.swap = true;
    state.set_pre_game(Some(pre_game));

    // handle pre_game setup immediately
    pre_game_internal(&app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
