use rayon::prelude::*;
use triple_triad::card::Card;
use triple_triad::game::{constants, Game};
use triple_triad::time::Stopwatch;

fn main() {
    // explore();
    // simple_explore();

    // simple_simulation();
    custom_simulation();

    // parallel_explore_max();
}

#[allow(dead_code)]
fn explore() {
    let mut stopwatch = Stopwatch::start();

    let mut game = Game::new();

    // game.rules.plus = true;

    // take turns to significantly cull search space
    let simulate_turns = 0;

    for _ in 0..simulate_turns {
        game = game.simulate_simple_turn();
    }

    game.start_explore();
    stopwatch.record("explore finished");
}

#[allow(dead_code)]
fn simple_explore() {
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

    game.computer.cards = [
        Card::computer("C0", 1, 1, 2, 1),
        Card::computer("C1", 1, 1, 1, 3),
        Card::computer("C2", 2, 1, 1, 1),
        Card::computer("C3", 4, 1, 4, 1),
        Card::computer("C4", 1, 1, 1, 1),
    ];

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

#[allow(dead_code)]
fn parallel_explore_max() {
    let square_count: u64 = 9;
    let moves_per_square = constants::total_depth_moves(0) / 9;
    let square_list: Vec<u64> = (0..square_count).collect();

    square_list.par_iter().for_each(|square| {
        let turn_start = *square * moves_per_square;
        let turn_end = (*square + 1) * moves_per_square;

        for turn in turn_start..turn_end {
            if turn % 10_000_000 == 0 {
                println!("{turn}");
            }
            // println!("{turn}");
        }
    });
}
