use tauri::Manager;

use crate::ui::AppState;
use crate::ui::AppStateJson;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn delete_deck(
    index: usize,
    app_handle: tauri::AppHandle,
) -> Result<AppStateJson, String> {
    return delete_deck_internal(index, &app_handle);
}

pub fn delete_deck_internal(
    index: usize,
    app_handle: &tauri::AppHandle,
) -> Result<AppStateJson, String> {
    let state = app_handle.state::<AppState>();

    let mut persist_data = state.persist_data.lock().unwrap().clone().unwrap();

    persist_data.deck_list.remove(index);

    persist_data.write(&app_handle);
    state.set_persist_data(Some(persist_data));

    // send back the updated game state
    Ok(state.json())
}
