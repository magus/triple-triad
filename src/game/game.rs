use rand::prelude::*;
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
        let start_turn = self.turn;

        let max_depth = 6;
        let depth = 0;

        self.explore(start_turn, max_depth, depth);
    }

    fn explore(&self, start_turn: u8, max_depth: u8, depth: u8) -> u64 {
        // println!("max_depth={max_depth}, start_turn={start_turn}, depth={depth}");

        let next_depth = depth + 1;
        // let depth = previous_depth + 1;
        // println!("[depth={depth}]");

        // find all valid moves from this game state and execute them
        let is_player = self.turn_is_player();
        let square_choices = self.squares_empty();
        let card_choices = if is_player {
            self.player.cards_left()
        } else {
            self.computer.cards_left()
        };

        let win_count_square = square_choices.par_iter().map(|square| {
            let win_count_card = card_choices.par_iter().map(|card| {
                // on turns 1 and 2 we cannot explore the full depth of search space
                // so we must cutoff threads by some heuristic such as player score
                // on turns 3+ we can fully explore the entire search space
                // and use win/loss ratio as our ultimate heuristic

                // in order to evalaute a branch of this depth-first search
                //
                //     heuristic_ratio = heuristic_count / total_iters
                //
                // total_iters: calculate for a given depth with math (based on is_player)
                // heuristic_count: evalaluate and count the number which meet heuristic
                // at depth 6 or 7 we can use heuristic_ratio to cutoff branches
                // preventing us from exploring them more deeply, reducing search space

                let game = self.execute_turn(is_player, *card, *square);

                // prevent going too deep on first few moves
                // this improves performance significantly
                //
                // worst case scenario, full depth of 8
                //     = 5 225 472 000 paths
                //     = 45 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1
                //
                // worst case scenario at depth of 5
                //     =   217 728 000 paths
                //     = 45 * 40 * 28 * 24 * 15 * 12
                //
                // once we have heuristics use it to cutoff instead of depth
                //
                if game.is_ended() {
                    // calculate heuristic (win) and pass it upward
                    // return 1;
                    let is_win = rand::random::<f32>() < 0.2;
                    return if is_win { 1 } else { 0 };
                } else if next_depth == max_depth {
                    // calculate heuristic (point) and pass it upward
                    // return 1;
                    let is_win = rand::random::<f32>() < 0.2;
                    return if is_win { 1 } else { 0 };
                } else {
                    return game.explore(start_turn, max_depth, next_depth);
                }
            });

            let win_count_card_sum: u64 = win_count_card.sum();
            return win_count_card_sum;
        });

        // calculate heuristic and pass it upward
        let win_count_square_sum: u64 = win_count_square.sum();

        // make decisions based on heuristics from one move ahead
        if depth == 1 {
            // println!("depth={depth}, start_turn={start_turn}, max_depth={max_depth}");
            let target_depth = start_turn + 1;
            let total_depth_moves = Game::total_depth_moves(target_depth);
            let max_depth_moves = Game::max_depth_moves(target_depth, max_depth);
            let score: f32 = 100.0 * (win_count_square_sum as f32 / max_depth_moves as f32);
            println!(
                "\n{:.4}%  ({win_count_square_sum} / {max_depth_moves}) [{total_depth_moves}]\n  {:?}",
                score, self.board,
            );
        }

        return win_count_square_sum;
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

    pub fn max_depth_moves(target_depth: u8, max_depth: u8) -> u64 {
        let is_exhaustive = TURN_MOVES.len() - (target_depth as usize) <= max_depth as usize;

        // println!(
        //     "target_depth={target_depth}, max_depth={max_depth}, is_exhaustive={is_exhaustive}"
        // );

        if max_depth == 0 || is_exhaustive {
            return Game::total_depth_moves(target_depth);
        }

        let mut total: u64 = 1;

        for i in 0..TURN_MOVES.len() {
            let depth = target_depth as usize + i;

            // println!("#{i} depth={depth}");

            if i == max_depth as usize - 1 {
                break;
            }

            if depth == TURN_MOVES.len() {
                break;
            }

            let mult = TURN_MOVES[depth] as u64;
            total *= mult;
            // println!("mult={mult} total={total}");
        }

        return total;
    }

    pub fn total_depth_moves(depth: u8) -> u64 {
        return DEPTH_MOVES[depth as usize];
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

const TURN_MOVES: [u8; 9] = [45, 40, 28, 24, 15, 12, 6, 4, 1];

const DEPTH_MOVES: [u64; 9] = [
    45 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1,
    40 * 28 * 24 * 15 * 12 * 6 * 4 * 1,
    28 * 24 * 15 * 12 * 6 * 4 * 1,
    24 * 15 * 12 * 6 * 4 * 1,
    15 * 12 * 6 * 4 * 1,
    12 * 6 * 4 * 1,
    6 * 4 * 1,
    4 * 1,
    1,
];
