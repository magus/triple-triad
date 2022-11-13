use tauri::Manager;

use crate::game::Rules;
use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn pre_game(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    return pre_game_internal(&app_handle);
}

pub fn pre_game_internal(app_handle: &tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let game = state.game.lock().unwrap().clone();

    let state_pre_game = state.pre_game.lock().unwrap().clone();

    // unwrap pre_game or default if None
    let pre_game = if state_pre_game.is_some() {
        state_pre_game.unwrap()
    } else {
        Rules {
            roulette: false,
            all_open: false,
            three_open: false,
            same: false,
            sudden_death: false,
            plus: false,
            random: false,
            order: false,
            chaos: false,
            reverse: false,
            fallen_ace: false,
            ascension: false,
            descension: false,
            swap: false,
            draft: false,
        }
    };

    state.set_pre_game(Some(pre_game));

    // check pre_game and compare against game.rules
    // ensure each rule is setup by setting it as status to handle in client
    if game.rules.all_open && !pre_game.all_open {
        state.set_status(Some("all_open".into()));
    } else {
        state.set_status(Some("turns".into()));
    }

    // send back the updated game state
    Ok(state.json())
}
