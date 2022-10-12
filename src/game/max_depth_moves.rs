use crate::game::constants::BOARD_SIZE;
use crate::game::Game;

impl Game {
    pub fn max_depth_moves(&self, target_depth: u8, input_max_depth: i8) -> u64 {
        let mut game = self.clone();

        let max_depth = if input_max_depth > -1 {
            input_max_depth as u8
        } else {
            BOARD_SIZE as u8
        };

        let end_turn = std::cmp::min(max_depth + game.turn + 1, BOARD_SIZE as u8);

        // println!();
        // println!();
        // println!("target_depth={target_depth}, max_depth={max_depth}, start_turn={}, end_turn={end_turn}", game.turn);

        for _ in game.turn..target_depth {
            // println!("simulating turn {i}");
            game = game.simulate_simple_turn();
        }

        let mut total_moves: u64 = 1;

        for _ in game.turn..end_turn {
            // println!("i={i}, turn={}, depth={depth}", game.turn);

            // find first valid move and make it
            let square_choices = game.squares_empty();
            let is_player = game.turn_is_player();

            let card_choices = if is_player {
                game.player.cards_left(self.chaos_card)
            } else {
                game.computer.cards_left()
            };

            game = game.execute_turn(card_choices[0], square_choices[0]);

            let turn_moves = (card_choices.len() * square_choices.len()) as u64;
            total_moves *= turn_moves;

            // println!("  turn_moves={turn_moves}");
            // println!("  total_moves={total_moves}");
        }

        // println!("max_depth={max_depth}, total_moves={total_moves}");
        return total_moves;
    }
}
