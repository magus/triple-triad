use rayon::prelude::*;
use triple_triad::game::Game;
use triple_triad::time::Stopwatch;

fn main() {
    explore();

    // simple_simulation();

    // parallel_explore_max();
}

#[allow(dead_code)]
fn explore() {
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
fn simple_simulation() {
    let mut game = Game::new();

    // simulate 10 turns in sequence to verify basic game flow
    for _turn in 1..10 {
        game = game.simulate_simple_turn();
    }

    println!("");
    println!("");
    println!("{:?}", game);
}

#[allow(dead_code)]
fn parallel_explore_max() {
    let square_count: u64 = 9;
    let moves_per_square = Game::total_depth_moves(0) / 9;
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
