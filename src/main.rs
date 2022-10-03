use rayon::prelude::*;
use triple_triad::game::Game;
use triple_triad::time::Stopwatch;

fn main() {
    let mut stopwatch = Stopwatch::start();

    let mut game = Game::new();
    // take turns to significantly cull search space
    for _ in 0..4 {
        game = game.simulate_simple_turn();
    }
    game.start_explore();
    println!("\n✅ done!");
    stopwatch.record("explore finished");

    // simple_simulation();

    // parallel_explore_max();
}

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

fn parallel_explore_max() {
    // 5 (card choices) to put into a single square
    // then * 40 ... for each choice after this turn
    let first_move_per_square: i64 = 5 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1;
    // let first_move_per_square: i64 = 2;

    let square_list: Vec<i64> = (0..9).collect();

    square_list.par_iter().for_each(|square| {
        let turn_start: i64 = *square * first_move_per_square;
        let turn_end: i64 = (*square + 1) * first_move_per_square;

        for turn in turn_start..turn_end {
            if turn % 10_000_000 == 0 {
                println!("{turn}");
            }
            // println!("{turn}");
        }
    });
}
