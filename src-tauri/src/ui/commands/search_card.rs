use tauri::Manager;

use crate::data::CardJson;
use crate::ui::AppState;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn search_card(
    search: &str,
    app_handle: tauri::AppHandle,
) -> Result<Vec<CardJson>, String> {
    let state = app_handle.state::<AppState>();

    let card_data_ref = state.card_data.lock();
    let card_data = card_data_ref.as_ref().unwrap().as_ref().unwrap();

    let card_results = card_data.search(search);

    let mut results = vec![];

    for card in card_results {
        results.push(card.clone());
    }

    Ok(results)
}
