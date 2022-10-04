use std::fmt;

use crate::card;
use crate::game::constants::BOARD_SIZE;
use crate::game::Game;

impl Game {
    fn print_board(&self) -> String {
        let mut parts: Vec<String> = vec![];

        parts.push(String::from("\n"));
        for i in 0..BOARD_SIZE {
            if i % 3 == 0 {
                parts.push(String::from("\n|"));
            }

            let card = self.board[i];
            let square = if card == card::EMPTY {
                format!("  {i}  ")
            } else {
                format!("{}({:?})", if card.is_player { "P" } else { "C" }, card)
            };

            parts.push(format!(" {} |", square));
        }
        parts.push(String::from("\n"));

        return parts.join("");
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print_board())
    }
}
