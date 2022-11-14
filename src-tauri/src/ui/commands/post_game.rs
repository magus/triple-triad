use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn post_game(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    return post_game_internal(&app_handle);
}

pub fn post_game_internal(app_handle: &tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let game = state.game.lock().unwrap().clone();

    let is_suddent_death = game.rules.sudden_death && game.score == 5 && game.is_ended();

    if !is_suddent_death {
        return Ok(state.json());
    }

    println!("☠️ Sudden Death");

    let mut player_cards = vec![];
    let mut computer_cards = vec![];

    for card in game.board {
        if card.is_player {
            player_cards.push(card);
        } else {
            // mark card guaranteed to optimize next run
            let mut edit_card = card.clone();
            edit_card.is_guaranteed = true;
            computer_cards.push(edit_card);
        }
    }

    for card in game.player.cards {
        if !card.is_empty {
            player_cards.push(card);
        }
    }

    for card in game.computer.cards {
        if !card.is_empty {
            computer_cards.push(card);
        }
    }

    let mut setup_game = state.setup_game.lock().unwrap().clone();
    setup_game.player.cards_from(player_cards);
    setup_game.computer.cards_from(computer_cards);

    // capture the game as-is for resetting
    state.set_setup_game(setup_game);

    // send back the updated game state
    Ok(state.json())
}
