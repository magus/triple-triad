use triple_triad::card;
use triple_triad::card::Card;
use triple_triad::data;
use triple_triad::game::Game;
use triple_triad::print;
use triple_triad::time::Stopwatch;

fn main() {
    // search_data();

    // print::drive_game_prompt();

    // TODO: implement chaos et al necessary to simulate "Idle Imperial"
    // Use it as a test to see if the recommendations work

    // guaranteed_card_left();

    idle_imperial();

    // deep_explore();

    // simple_simulation();
    // custom_simulation();
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn search_data() {
    let mut stopwatch = Stopwatch::start();

    let rule_data = data::RuleData::read();
    let card_data = data::CardData::read();
    let npc_data = data::NpcData::read(&card_data, &rule_data);

    // use npc data to allow selecting npc in prompt
    // use npc card data to set game cards with correct values
    // convert search input to lowercase for comparison
    println!("{:#?}", npc_data.find_all_npc("idle"));

    stopwatch.record("search_data finished");
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn guaranteed_card_left() {
    let mut stopwatch = Stopwatch::start();

    let mut game = Game::new();

    game.is_player_first = false;

    // game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3),
        Card::player("P1", 8, 2, 3, 8),
        Card::player("P2", 1, 8, 3, 8),
        Card::player("P3", 1, 5, 9, 9),
        Card::player("P4", 6, 10, 10, 1),
    ];

    game.computer.cards = game.computer.cards_from(vec![
        Card::computer_guaranteed("C0", 6, 3, 7, 3),
        Card::computer_guaranteed("C1", 9, 7, 8, 1),
        Card::computer("C2", 4, 1, 8, 7),
        Card::computer("C3", 7, 1, 6, 7),
        Card::computer("C4", 1, 4, 8, 8),
        Card::computer("C5", 7, 1, 3, 7),
        Card::computer("C6", 8, 3, 8, 1),
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

    stopwatch.record("guaranteed_card_left finished");
}

#[allow(dead_code)]
// https://arrtripletriad.com/en/npc-idle-imperial
fn idle_imperial() {
    let mut stopwatch = Stopwatch::start();

    let rule_data = data::RuleData::read();
    let card_data = data::CardData::read();
    let npc_data = data::NpcData::read(&card_data, &rule_data);

    let idle_imperial = npc_data.find_all_npc("idle").first().unwrap().clone();

    let mut game = Game::new();

    // game.is_player_first = false;

    // game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3),
        Card::player("P1", 8, 2, 3, 8),
        Card::player("P2", 1, 8, 3, 8),
        Card::player("P3", 1, 5, 9, 9),
        Card::player("P4", 6, 10, 10, 1),
    ];

    game.computer.cards = game.computer.cards_from(idle_imperial.cards.clone());

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
    stopwatch.record("explore finished");
}

#[allow(dead_code)]
fn deep_explore() {
    let mut stopwatch = Stopwatch::start();

    let mut game = Game::new();

    // take turns to significantly cull search space
    let simulate_turns = 0;

    for _ in 0..simulate_turns {
        game = game.simulate_simple_turn();
    }

    game.start_explore();
    stopwatch.record("explore finished");
}

#[allow(dead_code)]
fn custom_simulation() {
    let mut game = Game::new();

    game.rules.same = true;
    game.rules.plus = true;

    game.player.cards = [
        Card::player("P0", 1, 1, 1, 1),
        Card::player("P1", 1, 1, 1, 1),
        Card::player("P2", 1, 2, 1, 1),
        Card::player("P3", 1, 1, 1, 1),
        Card::player("P4", 1, 2, 3, 2),
    ];

    game.computer.cards = game.computer.cards_from(vec![
        Card::computer_guaranteed("C0", 1, 1, 2, 1),
        Card::computer_guaranteed("C1", 1, 1, 1, 3),
        Card::computer("C2", 2, 1, 1, 1),
        Card::computer("C3", 4, 1, 4, 1),
        Card::computer("C4", 1, 1, 1, 1),
        Card::computer("C5", 1, 1, 1, 1),
        Card::computer("C6", 1, 1, 1, 1),
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
