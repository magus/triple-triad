use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use crate::card::Card;

use super::start::start_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn set_deck(index: usize, app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    return set_deck_internal(index, &app_handle);
}

pub fn set_deck_internal(
    index: usize,
    app_handle: &tauri::AppHandle,
) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    let persist_data = state.persist_data.lock().unwrap().clone().unwrap();
    let deck = persist_data.deck_list[index].clone();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    let card_data_ref = state.card_data.lock();
    let card_data = card_data_ref.as_ref().unwrap().as_ref().unwrap();

    let mut cards = vec![];

    for id in deck.cards {
        let card = card_data.by_id(&id).unwrap();
        let id = card.id.parse::<u16>().unwrap();

        cards.push(Card::player(
            Card::player_name(cards.len()),
            id,
            card.top,
            card.right,
            card.bottom,
            card.left,
            card.tribe,
        ));
    }

    // sync cards back to game
    game.player.cards_from(cards.clone());

    // assign the game back to mutex
    state.set_game(game);

    // handle saving setup_game and reset
    start_internal(&app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
