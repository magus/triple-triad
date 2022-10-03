use rayon::prelude::*;

use crate::card;
use crate::card::Card;
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct Game {
    pub turn: u8,
    pub is_player_first: bool,
    pub board: [Card; 9],

    pub player: Player,
    pub computer: Player,
}

impl Game {
    pub fn start_explore(&self) {
        self.explore(1);
    }

    fn explore(&self, depth: u8) {
        // println!("[depth={depth}]");

        // find all valid moves from this game state and execute them
        let is_player = self.turn_is_player();
        let square_choices = self.squares_empty();
        let card_choices = if is_player {
            self.player.cards_left()
        } else {
            self.computer.cards_left()
        };

        square_choices.par_iter().for_each(|square| {
            card_choices.par_iter().for_each(|card| {
                // need to track whether this thread ends in a win or loss
                // we want branches where there is high likelihood of win

                let game = self.execute_turn(is_player, *card, *square);

                if !game.is_ended() {
                    // in worst case scenario this will explore
                    //   = 5 225 472 000 paths
                    //   = 45 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1
                    game.explore(depth + 1);
                }
            });
        });
    }

    pub fn execute_turn(&self, is_player: bool, card_index: usize, square_index: usize) -> Game {
        let mut game = self.clone();

        if is_player {
            let card = game.player.use_card(card_index);
            game.place_card(card, square_index);
        } else {
            let card = game.computer.use_card(card_index);
            game.place_card(card, square_index);
        };

        game.finish_turn();

        return game;
    }

    pub fn simulate_simple_turn(&self) -> Game {
        println!("");
        println!("");
        println!("[start turn {}] {}", self.turn + 1, self.player_name());
        println!("  [player.cards_left] {:?}", self.player.cards_left());
        // println!("  [player] [{:?}]", self.player.cards);
        println!("  [computer.cards_left] {:?}", self.computer.cards_left());
        // println!("  [computer] [{:?}]", self.computer.cards);
        println!("  [squares_empty] {:?}", self.squares_empty());
        // println!("  [board] {:?}", self.board);

        // find first valid move and make it
        let square_choices = self.squares_empty();
        let is_player = self.turn_is_player();

        let card_choices = if is_player {
            self.player.cards_left()
        } else {
            self.computer.cards_left()
        };

        let game = self.execute_turn(is_player, card_choices[0], square_choices[0]);

        println!("[end turn {}]", game.turn);
        println!("  [player.cards_left] {:?}", game.player.cards_left());
        // println!("  [player] [{:?}]", game.player.cards);
        println!("  [computer.cards_left] {:?}", game.computer.cards_left());
        // println!("  [computer] [{:?}]", game.computer.cards);
        println!("  [squares_empty] {:?}", game.squares_empty());
        // println!("  [board] {:?}", game.board);

        return game;
    }

    pub fn squares_empty(&self) -> Vec<usize> {
        let mut card_index_list = vec![];

        for i in 0..9 {
            if self.board[i] == card::EMPTY {
                card_index_list.push(i);
            }
        }

        return card_index_list;
    }

    pub fn place_card(&mut self, card: Card, index: usize) -> bool {
        if card != card::EMPTY && self.board[index] == card::EMPTY {
            // place the card in this board square
            self.board[index] = card;
            return true;
        }

        return false;
    }

    pub fn finish_turn(&mut self) {
        self.turn += 1;
    }

    pub fn turn_is_player(&self) -> bool {
        let remainder: u8 = if self.is_player_first { 0 } else { 1 };
        return self.turn % 2 == remainder;
    }

    pub fn player_name(&self) -> &str {
        if self.turn_is_player() {
            "player"
        } else {
            "computer"
        }
    }

    pub fn is_ended(&self) -> bool {
        return self.turn == 9;
    }

    pub fn new() -> Self {
        let board = [
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
            card::EMPTY,
        ];

        let cards = [
            Card {
                name: "P1",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "P2",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "P3",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "P4",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "P5",
                sides: (1, 1, 1, 1),
            },
        ];

        let computer_cards = [
            Card {
                name: "C1",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "C2",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "C3",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "C4",
                sides: (1, 1, 1, 1),
            },
            Card {
                name: "C5",
                sides: (1, 1, 1, 1),
            },
        ];

        let player = Player {
            is_player: true,
            cards,
        };

        let computer = Player {
            is_player: false,
            cards: computer_cards,
        };

        return Game {
            turn: 0,
            is_player_first: true,
            board,

            player,
            computer,
        };
    }
}
