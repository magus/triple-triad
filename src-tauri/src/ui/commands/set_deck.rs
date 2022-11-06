use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

use crate::card::Card;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn set_deck(app_handle: tauri::AppHandle) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    // grab game via mutex and clone for mutating and reassigning back to mutex
    let mut game = state.game.lock().unwrap().clone();

    let card_data_ref = state.card_data.lock();
    let card_data = card_data_ref.as_ref().unwrap().as_ref().unwrap();

    let card_list = vec![
        card_data.by_id("88").unwrap(),
        card_data.by_id("75").unwrap(),
        card_data.by_id("89").unwrap(),
        card_data.by_id("93").unwrap(),
        card_data.by_id("96").unwrap(),
    ];

    let mut cards = vec![];

    for card in card_list {
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

    // send back the updated game state
    Ok(state.json())
}
