use tauri::Manager;

use crate::ui::persist_data::Deck;
use crate::ui::AppState;
use crate::ui::AppStateJson;

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

    persist_data.write(&app_handle);

    state.set_persist_data(Some(persist_data));

    // send back the updated game state
    Ok(state.json())
}
