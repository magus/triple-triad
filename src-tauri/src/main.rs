#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager};

use triple_triad::ui;
use triple_triad::ui::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .setup(app_main)
        .invoke_handler(tauri::generate_handler![
            ui::commands::greet::greet,
            ui::commands::state::state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn app_main(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // only include this code on debug builds
    #[cfg(debug_assertions)]
    {}

    // always open devtools when app is setup
    let window = app.get_window("main").unwrap();
    window.open_devtools();

    // TODO
    // - use json of AppState.game to display board
    // - store CardIdLookup in AppState
    //      - map from card.name (e.g. C2,P0, etc) to card id for displaying card images
    //
    // - display game rules
    //      - display disabled rules dimmed / opacity
    //      - images in data/game/data/ui/icon/121000/en/121611_hr1.png - 121624_hr1.png
    //
    // - use invoke to retrieve game state and keep in sync
    // - invoke to mutate game state and execute turns, etc.
    //
    // - setup pages for various tasks (e.g. select npc, input cards, etc.)
    //
    //
    // - game simulation flow
    //      - setup game
    //          - select card deck (allow creating new decks here)
    //              - input card deck
    //          - select npc opponent
    //      - pre game
    //          - select first player
    //          - swap, open, etc.
    //      - alternate game turns
    //      - post game
    //          - suddent death, etc.
    //      - rematch?
    //

    Ok(())
}
