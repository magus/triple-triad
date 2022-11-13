#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager};

#[allow(unused_imports)]
use triple_triad::debug;
use triple_triad::ui;
use triple_triad::ui::AppState;

fn main() {
    // debug::idle_imperial();
    // return;

    tauri::Builder::default()
        .manage(AppState::new())
        .setup(app_main)
        .invoke_handler(tauri::generate_handler![
            ui::commands::status::status,
            ui::commands::state::state,
            ui::commands::search_card::search_card,
            ui::commands::search_npc::search_npc,
            ui::commands::save_deck::save_deck,
            ui::commands::set_deck::set_deck,
            ui::commands::set_npc::set_npc,
            ui::commands::start::start,
            ui::commands::pre_game::pre_game,
            ui::commands::roulette::roulette,
            ui::commands::all_open::all_open,
            ui::commands::three_open::three_open,
            ui::commands::swap::swap,
            ui::commands::explore::explore,
            ui::commands::execute_turn::execute_turn,
            ui::commands::chaos_select::chaos_select,
            ui::commands::post_game::post_game,
            ui::commands::reset::reset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn app_main(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // only include this code on debug builds
    #[cfg(debug_assertions)]
    {}

    app.state::<AppState>().init_data(app);

    // always open devtools when app is setup
    let window = app.get_window("main").unwrap();
    window.open_devtools();

    Ok(())
}
