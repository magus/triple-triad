use rayon::prelude::*;
use triple_triad::game::Game;

fn main() {
    let mut game = Game::new();

    for _turn in 1..10 {
        game = game.execute_turn();
    }

    println!("");
    println!("");
    println!("{:?}", game);

    // 5 (card choices) to put into a single square
    // then * 40 ... for each choice after this turn
    let first_move_per_square: i64 = 5 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1;
    // let first_move_per_square: i64 = 2;

    let mut square_list: Vec<i64> = (0..9).collect();

    // square_list.par_iter_mut().for_each(|square| {
    //     let turn_start: i64 = *square * first_move_per_square;
    //     let turn_end: i64 = (*square + 1) * first_move_per_square;

    //     for turn in turn_start..turn_end {
    //         if turn % 10_000_000 == 0 {
    //             println!("{turn}");
    //         }
    //         // println!("{turn}");
    //     }
    // });
}
