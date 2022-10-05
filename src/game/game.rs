use rayon::prelude::*;

use crate::card;
use crate::card::Card;
use crate::game::constants::{self, BOARD_SIZE};
use crate::game::impact::{self, ImpactPair};
use crate::player::Player;

type Board = [Card; BOARD_SIZE];

#[derive(Clone)]
pub struct Game {
    pub turn: u8,
    pub is_player_first: bool,
    pub score: i8,
    pub board: Board,

    pub player: Player,
    pub computer: Player,
}

impl Game {
    pub fn start_explore(&self) {
        let start_turn = self.turn;
        let depth = 0;

        // on turn 1 the search space is worst case scenario
        //
        //     = 5 225 472 000 paths
        //     = 45 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1
        //
        // so we set max_depth to 6 in order to significantly
        // improve performance by cutting search space
        //
        //     =   217 728 000 paths
        //     = 45 * 40 * 28 * 24 * 15 * 12
        //
        // once we have heuristics use it to cutoff instead of depth
        // at max_depth we can instead use the score of a game to decide
        // whether to recurse with game.explore, again culling search space
        let max_depth = if start_turn > 0 { -1 } else { 6 };

        println!("\n📊 results");
        self.explore(start_turn, max_depth, depth);

        println!(
            "\n✅ done [{} paths evaluated]",
            constants::max_depth_moves(start_turn, max_depth + 1)
        );
    }

    fn explore(&self, start_turn: u8, max_depth: i8, depth: u8) -> f64 {
        // println!("max_depth={max_depth}, start_turn={start_turn}, depth={depth}");

        let target_depth = start_turn + 1;
        let next_depth = depth + 1;

        // println!("target_depth={target_depth}, depth={depth}, is_exhaustive={is_exhaustive}");

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
                let game = self.execute_turn(is_player, *card, *square);

                if game.is_ended() {
                    // pass win-loss back as 0 or 1
                    let is_win = rand::random::<f64>() < 0.2;
                    return if is_win { 1.0 } else { 0.0 };
                } else if next_depth as i8 == max_depth {
                    // println!("next_depth={next_depth}, max_depth={max_depth}");

                    // evaluate board position as cards flipped for player
                    // for example, `N / game.turn` will return a float in range [0,1]
                    // this can be summed in the same way as win-loss above
                    // let player_evaluation = rand::random::<f64>();
                    // return player_evaluation;
                    let is_win = rand::random::<f64>() < 0.2;
                    return if is_win { 1.0 } else { 0.0 };
                } else {
                    return game.explore(start_turn, max_depth, next_depth);
                }
            });

            let win_count_card_sum: f64 = win_count_card.sum();
            return win_count_card_sum;
        });

        // calculate heuristic and pass it upward
        let total_wins: f64 = win_count_square.sum();

        // make decisions based on heuristics from one move ahead
        if depth == 1 {
            // println!("depth={depth}, start_turn={start_turn}, max_depth={max_depth}");

            let total_depth_moves = constants::total_depth_moves(target_depth);
            let max_depth_moves = constants::max_depth_moves(target_depth, max_depth);
            let score = 100.0 * (total_wins / max_depth_moves as f64);
            println!(
                "\n{:.4}%  ({total_wins} / {max_depth_moves}) [{total_depth_moves}]\n  {:?}",
                score, self.board,
            );
        }

        return total_wins;
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
        println!();
        println!();

        if self.is_ended() {
            println!("  [game ended]");
            return self.clone();
        }

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

        for i in 0..BOARD_SIZE {
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

            let is_combo = false;
            self.card_impact(index, is_combo);
            self.score += if self.turn_is_player() { 1 } else { 0 };

            return true;
        }

        return false;
    }

    pub fn flip(&mut self, square: usize) -> bool {
        let is_player = self.turn_is_player();
        let is_flip = self.board[square].is_player != is_player;

        if is_flip {
            // println!("♻️ flip! [{square}] for [{}]", self.player_name());

            self.board[square].flip(is_player);
            self.score += if is_player { 1 } else { -1 };
        }

        return is_flip;
    }

    pub fn flip_all(&mut self, squares: &Vec<usize>) {
        for square in squares {
            self.flip(*square);
        }
    }

    pub fn is_flip(&self, placed: u8, impacted: u8) -> bool {
        // TODO take into consideration rules

        if placed > impacted {
            return true;
        }

        return false;
    }

    // pub fn check_sides(&self, sides: HashMap<&str, ImpactPair>) {
    pub fn check_sides(&mut self, sides: [Option<ImpactPair>; 4], is_combo: bool) {
        // println!(
        //     "{} check_sides {:?}",
        //     if is_combo { "COMBO!" } else { "" },
        //     sides
        // );

        let mut combo_squares = vec![];

        // 1. check same/plus pairs first for combo squares
        //    skip this step if we are already in a combo
        if !is_combo {
            let mut pair_flips = self.check_pairs(sides);
            self.flip_all(&pair_flips);
            combo_squares.append(&mut pair_flips);
        }

        // 2. then check and flip normal sides
        //    making sure to include them in combo as well
        for pair in sides {
            if let Some(pair) = pair {
                if self.is_flip(pair.card, pair.impacted) {
                    if self.flip(pair.square) {
                        combo_squares.push(pair.square);
                    }
                }
            }
        }

        // 3. finally recurse by calling card_impact for each combo square
        //    which was flipped by either
        //       - same/plus pair above or
        //       - normal flip while is_combo is true
        // println!("combo_squares={:?}", combo_squares);
        for square in combo_squares {
            let is_combo = true;
            self.card_impact(square, is_combo);
        }
    }

    // https://www.reddit.com/r/ffxiv/comments/p54gkq/ahhh_exactly_what_i_wanted_to_do/
    // each plus/same flipped card can create a combo
    // this means it can flip the impacted cards of the impacted card (ignoring repeat plus/same rules)
    // this process continues recursively until there are no flips
    // we can implement this by calling card_impact on plus/same flipped cards with the combo flag
    // pub fn check_pairs(&self, sides: HashMap<&str, ImpactPair>) -> Vec<usize> {
    pub fn check_pairs(&self, sides: [Option<ImpactPair>; 4]) -> Vec<usize> {
        let mut flips = vec![];

        if let [Some(top), Some(right), Some(bottom), Some(left)] = sides {
            // T | R | B | L
            // 1 | 1 | 1 | 1
            if top.sum == right.sum && bottom.sum == left.sum && left.sum == right.sum {
                flips = vec![top.square, right.square, bottom.square, left.square];
            }
        } else if let [_, Some(right), Some(bottom), Some(left)] = sides {
            // T | R | B | L
            // 0 | 1 | 1 | 1
            if right.sum == bottom.sum && bottom.sum == left.sum {
                flips = vec![right.square, bottom.square, left.square];
            }
        } else if let [Some(top), _, Some(bottom), Some(left)] = sides {
            // T | R | B | L
            // 1 | 0 | 1 | 1
            if top.sum == bottom.sum && bottom.sum == left.sum {
                flips = vec![top.square, bottom.square, left.square];
            }
        } else if let [Some(top), Some(right), _, Some(left)] = sides {
            // T | R | B | L
            // 1 | 1 | 0 | 1
            if top.sum == right.sum && right.sum == left.sum {
                flips = vec![top.square, right.square, left.square];
            }
        } else if let [Some(top), Some(right), Some(bottom), _] = sides {
            // T | R | B | L
            // 1 | 1 | 1 | 0
            if top.sum == right.sum && right.sum == bottom.sum {
                flips = vec![top.square, right.square, bottom.square];
            }
        } else if let [Some(top), Some(right), _, _] = sides {
            // T | R | B | L
            // 1 | 1 | 0 | 0
            if top.sum == right.sum {
                flips = vec![top.square, right.square];
            }
        } else if let [_, Some(right), Some(bottom), _] = sides {
            // T | R | B | L
            // 0 | 1 | 1 | 0
            if right.sum == bottom.sum {
                flips = vec![right.square, bottom.square];
            }
        } else if let [_, _, Some(bottom), Some(left)] = sides {
            // T | R | B | L
            // 0 | 0 | 1 | 1
            if bottom.sum == left.sum {
                flips = vec![bottom.square, left.square];
            }
        } else if let [Some(top), _, _, Some(left)] = sides {
            // T | R | B | L
            // 1 | 0 | 0 | 1
            if top.sum == left.sum {
                flips = vec![top.square, left.square];
            }
        }

        return flips;
    }

    pub fn card_impact(&mut self, index: usize, is_combo: bool) {
        let card = self.board[index];

        let mut sides = [None, None, None, None];

        let impacts = impact::BOARD[index];

        for i_impact in 0..impacts.len() {
            let impact = impacts[i_impact];

            if let Some(square) = impact {
                match self.board[square] {
                    // if square is empty we do nothing
                    card::EMPTY => {}

                    impacted => {
                        // handle [top, right, bottom, left]
                        match i_impact {
                            0 => {
                                sides[0] =
                                    Some(ImpactPair::new(square, card.top(), impacted.bottom()));
                            }
                            1 => {
                                sides[1] =
                                    Some(ImpactPair::new(square, card.right(), impacted.left()));
                            }
                            2 => {
                                sides[2] =
                                    Some(ImpactPair::new(square, card.bottom(), impacted.top()));
                            }
                            3 => {
                                sides[3] =
                                    Some(ImpactPair::new(square, card.left(), impacted.right()));
                            }
                            _ => panic!("unhandled card_impact i_impact [{}]", i_impact),
                        };
                    }
                }
            }
        }

        // examine each pair now that we've collected them into vec
        self.check_sides(sides, is_combo);
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
        return self.turn == BOARD_SIZE as u8;
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

        let player = Player {
            is_player: true,
            cards: [
                Card::player("P1", 2, 2, 2, 2),
                Card::player("P2", 2, 2, 2, 2),
                Card::player("P3", 2, 2, 2, 2),
                Card::player("P4", 2, 2, 2, 2),
                Card::player("P5", 2, 2, 2, 2),
            ],
        };

        let computer = Player {
            is_player: false,
            cards: [
                Card::computer("C1", 1, 1, 1, 1),
                Card::computer("C2", 1, 1, 1, 1),
                Card::computer("C3", 1, 1, 1, 1),
                Card::computer("C4", 1, 1, 1, 1),
                Card::computer("C5", 1, 1, 1, 1),
            ],
        };

        return Game {
            turn: 0,
            is_player_first: true,
            score: 0,
            board,

            player,
            computer,
        };
    }
}
