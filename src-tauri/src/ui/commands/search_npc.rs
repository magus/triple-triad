use tauri::Manager;

use crate::data::Npc;
use crate::ui::AppState;

// see https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn search_npc(search: &str, app_handle: tauri::AppHandle) -> Result<Vec<Npc>, String> {
    let state = app_handle.state::<AppState>();

    let npc_data_ref = state.npc_data.lock();
    let npc_data = npc_data_ref.as_ref().unwrap().as_ref().unwrap();

    let npc_results = npc_data.search(search);

    let mut results = vec![];

    for npc in npc_results {
        results.push(npc.clone());
    }

    Ok(results)
}
