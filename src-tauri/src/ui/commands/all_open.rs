use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use super::pre_game::pre_game_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn all_open(
    cards: Vec<usize>,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    // println!("[command::all_open] card={:#?}", cards);

    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    let mut selected_cards = vec![];

    for card in game.computer.cards {
        if card.is_guaranteed {
            selected_cards.push(card);
        }
    }

    for index in cards {
        let mut card = game.computer.cards[index];
        card.is_guaranteed = true;
        selected_cards.push(card);
    }

    game.computer.cards_from(selected_cards);

    state.set_game(game);

    let mut pre_game = state.pre_game.lock().unwrap().clone().unwrap();
    pre_game.all_open = true;
    state.set_pre_game(Some(pre_game));

    // handle pre_game setup immediately
    pre_game_internal(&app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
