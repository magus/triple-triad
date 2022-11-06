use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn execute_turn(
    card: usize,
    square: usize,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    println!("[command::execute_turn] card={card} square={square}");

    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    game = game.execute_turn(card, square);

    // assign the game back to mutex
    state.set_game(game);

    // send back the updated game state
    Ok(state.json())
}
