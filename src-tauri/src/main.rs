#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(unused_imports)]
use triple_triad::card;
use triple_triad::card::Card;
use triple_triad::data;
use triple_triad::game::Game;
#[allow(unused_imports)]
use triple_triad::print;
use triple_triad::time::Stopwatch;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // print::drive_game_prompt();

    // search_data();

    // TODO: implement chaos et al necessary to simulate "Idle Imperial"
    // Use it as a test to see if the recommendations work

    // guaranteed_card_left();

    // idle_imperial();

    // deep_explore();

    // simple_simulation();
    // custom_simulation();
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn search_data() {
    let mut stopwatch = Stopwatch::start();

    let rule_data = data::RuleData::read();
    stopwatch.record("rule data loaded");
    let card_data = data::CardData::read();
    stopwatch.record("card data loaded");
    let npc_data = data::NpcData::read(&card_data, &rule_data);
    stopwatch.record("npc data loaded");

    // use npc data to allow selecting npc in prompt
    // use npc card data to set game cards with correct values
    // convert search input to lowercase for comparison

    println!("{:#?}", npc_data.search("idle"));
    stopwatch.record("npc_data.search(\"idle\")");

    println!("{:#?}", card_data.search("arm"));
    stopwatch.record("card_data.search(\"arm\")");
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn guaranteed_card_left() {
    let mut game = Game::new();

    game.is_player_first = false;

    // game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3, Card::TRIBE_NONE),
        Card::player("P1", 8, 2, 3, 8, Card::TRIBE_NONE),
        Card::player("P2", 1, 8, 3, 8, Card::TRIBE_NONE),
        Card::player("P3", 1, 5, 9, 9, Card::TRIBE_NONE),
        Card::player("P4", 6, 10, 10, 1, Card::TRIBE_NONE),
    ];

    game.computer.cards_from(vec![
        Card::computer_guaranteed("C0", 6, 3, 7, 3, Card::TRIBE_NONE),
        Card::computer_guaranteed("C1", 9, 7, 8, 1, Card::TRIBE_NONE),
        Card::computer("C2", 4, 1, 8, 7, Card::TRIBE_NONE),
        Card::computer("C3", 7, 1, 6, 7, Card::TRIBE_NONE),
        Card::computer("C4", 1, 4, 8, 8, Card::TRIBE_NONE),
        Card::computer("C5", 7, 1, 3, 7, Card::TRIBE_NONE),
        Card::computer("C6", 8, 3, 8, 1, Card::TRIBE_NONE),
    ]);

    // computer
    game = game.execute_turn(2, 8);
    println!("{:?}", game);

    // player
    game = game.execute_turn(4, 2);
    println!("{:?}", game);

    // computer
    game = game.execute_turn(3, 7);
    println!("{:?}", game);

    // player
    game = game.execute_turn(0, 4);
    println!("{:?}", game);

    // computer
    game = game.execute_turn(4, 6);
    println!("{:?}", game);

    // cards are guaranteed to be 0 and 1 at this point
    // since they have not been used but are guaranteed
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn idle_imperial() {
    let rule_data = data::RuleData::read();
    let card_data = data::CardData::read();
    let npc_data = data::NpcData::read(&card_data, &rule_data);

    let idle_imperial = npc_data.search("idle").first().unwrap().clone();

    let mut game = Game::new();

    // game.is_player_first = false;

    // game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3, Card::TRIBE_NONE),
        Card::player("P1", 8, 2, 3, 8, Card::TRIBE_NONE),
        Card::player("P2", 1, 8, 3, 8, Card::TRIBE_NONE),
        Card::player("P3", 1, 5, 9, 9, Card::TRIBE_NONE),
        Card::player("P4", 6, 10, 10, 1, Card::TRIBE_NONE),
    ];

    game.computer.cards_from(idle_imperial.cards.clone());

    // player
    game = game.execute_turn(1, 8);
    println!("{:?}", game);

    // // computer
    // game = game.execute_turn(4, 2);
    // println!("{:?}", game);

    // // player
    // game = game.execute_turn(2, 7);
    // println!("{:?}", game);

    // // computer
    // game = game.execute_turn(0, 4);
    // println!("{:?}", game);

    // // player
    // game = game.execute_turn(3, 6);
    // println!("{:?}", game);

    // // computer
    // game = game.execute_turn(1, 3);
    // println!("{:?}", game);

    // // player
    // game = game.execute_turn(0, 0);
    // println!("{:?}", game);

    // // computer
    // game = game.execute_turn(2, 1);
    // println!("{:?}", game);

    // // player
    // game = game.execute_turn(4, 5);
    // println!("{:?}", game);

    game.start_explore();
}

#[allow(dead_code)]
fn deep_explore() {
    let mut game = Game::new();

    // take turns to significantly cull search space
    let simulate_turns = 0;

    for _ in 0..simulate_turns {
        game = game.simulate_simple_turn();
    }

    game.start_explore();
}

#[allow(dead_code)]
fn custom_simulation() {
    let mut game = Game::new();

    game.rules.same = true;
    game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 1, 1, 1, 1, Card::TRIBE_NONE),
        Card::player("P1", 1, 1, 1, 1, Card::TRIBE_NONE),
        Card::player("P2", 1, 2, 1, 1, Card::TRIBE_NONE),
        Card::player("P3", 1, 1, 1, 1, Card::TRIBE_NONE),
        Card::player("P4", 1, 2, 3, 2, Card::TRIBE_NONE),
    ];

    game.computer.cards_from(vec![
        Card::computer_guaranteed("C0", 1, 1, 2, 1, Card::TRIBE_NONE),
        Card::computer_guaranteed("C1", 1, 1, 1, 3, Card::TRIBE_NONE),
        Card::computer("C2", 2, 1, 1, 1, Card::TRIBE_NONE),
        Card::computer("C3", 4, 1, 4, 1, Card::TRIBE_NONE),
        Card::computer("C4", 1, 1, 1, 1, Card::TRIBE_NONE),
        Card::computer("C5", 1, 1, 1, 1, Card::TRIBE_NONE),
        Card::computer("C6", 1, 1, 1, 1, Card::TRIBE_NONE),
    ]);

    game = game.execute_turn(0, 0);
    println!("{:?}", game);

    game = game.execute_turn(0, 1);
    println!("{:?}", game);

    game = game.execute_turn(1, 2);
    println!("{:?}", game);

    game = game.execute_turn(1, 5);
    println!("{:?}", game);

    game = game.execute_turn(2, 6);
    println!("{:?}", game);

    game = game.execute_turn(2, 7);
    println!("{:?}", game);

    game = game.execute_turn(3, 8);
    println!("{:?}", game);

    game = game.execute_turn(3, 3);
    println!("{:?}", game);

    game = game.execute_turn(4, 4);
    println!("{:?}", game);
}

#[allow(dead_code)]
fn simple_simulation() {
    let mut game = Game::new();

    // simulate 10 turns in sequence to verify basic game flow
    for _turn in 1..10 {
        println!("{:?}", game);
        game = game.simulate_simple_turn();
    }

    println!("{:?}", game);
}