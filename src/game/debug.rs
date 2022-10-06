use colored::*;
use std::fmt;

use crate::card::Card;
use crate::game::constants::ROW_SIZE;
use crate::game::Game;

fn print_card_color(card: Card, text: String) -> String {
    let ((t_r, t_g, t_b), (r, g, b)) = card.rgb_color();

    return text
        .truecolor(t_r, t_g, t_b)
        .on_truecolor(r, g, b)
        .to_string();
}

impl Game {
    fn print_board_row(&self, row: usize) {
        let row_start = row * ROW_SIZE;
        let row_end = (row + 1) * ROW_SIZE;

        println!();

        for i in row_start..row_end {
            let card = self.board[i];

            print!(
                "{}",
                print_card_color(card, format!("{}          ", card.name))
            );
            print!(" ");
        }

        println!();

        for i in row_start..row_end {
            let card = self.board[i];

            print!(
                "{}",
                print_card_color(card, format!("     {}      ", card.print_top()))
            );
            print!(" ");
        }

        println!();

        for i in row_start..row_end {
            let card = self.board[i];

            print!(
                "{}",
                print_card_color(
                    card,
                    format!("  {}     {}   ", card.print_left(), card.print_right())
                )
            );
            print!(" ");
        }

        println!();

        for i in row_start..row_end {
            let card = self.board[i];
            print!(
                "{}",
                print_card_color(card, format!("     {}      ", card.print_bottom()))
            );
            print!(" ");
        }

        println!();

        for i in row_start..row_end {
            let card = self.board[i];
            print!("{}", print_card_color(card, format!("           {}", i)));
            print!(" ");
        }
    }

    fn print_board(&self) {
        // print diamond of sides for better card side visuals
        //
        // example
        // 1. print the top of all cards in top row
        // 2. print the left/right of all cards in top row
        // 3. print the bottom of all cards in top row
        // 4. repeat for middle row
        // 5. repeat for bottom row
        //
        //   T      T      T
        // L   R  L   R  L   R
        //   B      B      B
        //
        // end result should be easier to read/reason about

        println!();
        self.print_board_row(0);
        println!();
        self.print_board_row(1);
        println!();
        self.print_board_row(2);
        println!();
        println!();
        println!("==========================================");
        println!();
    }

    fn print_turn(&self) -> String {
        if self.is_ended() {
            return format!("{}", if self.is_win() { "WIN" } else { "LOSE" });
        }

        return format!("Turn: {}", self.turn);
    }

    fn print_score(&self) -> String {
        let percent = self.percent_score();
        return format!("Score: {:.2}% ({} / {})", percent, self.score, self.turn);
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = write!(f, "{}", [self.print_turn(), self.print_score()].join("\n"));
        self.print_board();
        return result;
    }
}
