use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn execute_turn(
    card: usize,
    square: usize,
    is_player: bool,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    println!("[command::execute_turn] card={card} square={square} is_player={is_player}");

    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    // special case for first turn we allow either player to go first
    // this keeps the ui simple, we can immediately update first_player
    // before executing the turn to ensure the correct state
    if game.turn == 0 {
        game.is_player_first = is_player;
    }

    game = game.execute_turn(card, square);

    // assign the game back to mutex
    state.set_game(game);

    // send back the updated game state
    Ok(state.json())
}
