use tauri::Manager;

use crate::ui::persist_data::Deck;
use crate::ui::AppState;
use crate::ui::AppStateJson;

use super::set_deck::set_deck_internal;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn save_deck(
    cards: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    // println!("[command::all_open] card={:#?}", cards);

    let state = app_handle.state::<AppState>();

    let mut persist_data = state.persist_data.lock().unwrap().clone().unwrap();
    persist_data.deck_list.push(Deck { cards });
    let index = persist_data.deck_list.len() - 1;

    persist_data.write(&app_handle);

    state.set_persist_data(Some(persist_data));

    set_deck_internal(index, &app_handle)?;

    // send back the updated game state
    Ok(state.json())
}
