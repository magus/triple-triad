use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::card;
use crate::card::Card;
use crate::game::constants::{self, BOARD_SIZE};
use crate::game::impact::{self, ImpactPair};
use crate::player::Player;

type Board = [Card; BOARD_SIZE];

#[derive(Copy, Clone, Debug)]
pub struct Rules {
    pub plus: bool,
    pub same: bool,
}

#[derive(Clone)]
pub struct Game {
    pub turn: u8,
    pub is_player_first: bool,
    pub score: i8,
    pub rules: Rules,
    pub board: Board,
    pub last_move: i8,

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

        // collect results across threads into shard vector
        let results: Arc<Mutex<Vec<(f64, Game)>>> = Arc::new(Mutex::new(Vec::new()));

        self.explore(start_turn, max_depth, depth, &results);

        println!("\n📊 results");

        // sort results by comparing score values
        let mut safe_results = results.lock().unwrap();
        safe_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // show up to top 3 moves
        let show_count = std::cmp::min(3, safe_results.len());

        for i in 0..show_count {
            let (score, game) = &safe_results[i];
            let total_depth_moves = constants::total_depth_moves(game.turn);
            let max_depth_moves = constants::max_depth_moves(game.turn, max_depth);

            let is_estimate = max_depth_moves < total_depth_moves;

            if is_estimate {
                println!("\n{:.4}% chance to win", score);
                println!("{max_depth_moves} moves evaluated (out of {total_depth_moves} possible)");
            } else {
                println!("\n{:.4}% chance to win", score);
                println!("{total_depth_moves} moves evaluated");
            }

            println!();
            println!("{:?}", game);

            println!("========================");
        }

        println!(
            "\n✅ done [{} paths evaluated]",
            constants::max_depth_moves(start_turn, max_depth + 1)
        );
    }

    fn explore(
        &self,
        start_turn: u8,
        max_depth: i8,
        depth: u8,
        results: &Arc<Mutex<Vec<(f64, Game)>>>,
    ) -> f64 {
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

        let total_moves = square_choices.par_iter().map(|square| {
            let total_square = card_choices.par_iter().map(|card| {
                let game = self.execute_turn(*card, *square);

                if game.is_ended() {
                    // pass win-loss back as 0 or 1
                    let is_win = rand::random::<f64>() < 0.2;
                    return if is_win { 100.0 } else { 0.0 };
                } else if next_depth as i8 == max_depth {
                    // println!("next_depth={next_depth}, max_depth={max_depth}");

                    // evaluate board position as cards flipped for player
                    // for example, `N / game.turn` will return a float in range [0,1]
                    // this can be summed in the same way as win-loss above
                    return self.percent_score();
                } else {
                    return game.explore(start_turn, max_depth, next_depth, results);
                }
            });

            let total_square_sum: f64 = total_square.sum();
            return total_square_sum;
        });

        // calculate heuristic and pass it upward
        let total_score: f64 = total_moves.sum();

        // make decisions based on heuristics from one move ahead
        if depth == 1 {
            let max_depth_moves = constants::max_depth_moves(target_depth, max_depth);
            let score = total_score / max_depth_moves as f64;

            results.lock().unwrap().push((score, self.clone()));
        }

        return total_score;
    }

    pub fn execute_turn(&self, card_index: usize, square_index: usize) -> Game {
        let mut game = self.clone();

        if self.turn_is_player() {
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

        let game = self.execute_turn(card_choices[0], square_choices[0]);

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
            self.last_move = index as i8;

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
            if self.rules.plus {
                let mut pair_flips = self.check_pairs(sides, plus_pairs);
                self.flip_all(&pair_flips);
                combo_squares.append(&mut pair_flips);
            }

            if self.rules.same {
                let mut pair_flips = self.check_pairs(sides, same_pairs);
                self.flip_all(&pair_flips);
                combo_squares.append(&mut pair_flips);
            }
        }

        // 2. then check and flip normal sides
        //    making sure to include them in combo as well
        for pair in sides {
            if let Some(pair) = pair {
                if self.is_flip(pair.card, pair.impacted) {
                    if self.flip(pair.square) {
                        // if we are in combo then normal flips can continue the combo
                        if is_combo {
                            combo_squares.push(pair.square);
                        }
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

    pub fn is_valid_plus_same(&self, squares: &Vec<usize>) -> bool {
        let is_player = self.turn_is_player();

        // plus and same require at least one card in group to belong to opponent
        return squares
            .iter()
            .any(|&square| self.board[square].is_player != is_player);
    }

    // https://www.reddit.com/r/ffxiv/comments/p54gkq/ahhh_exactly_what_i_wanted_to_do/
    // each plus/same flipped card can create a combo
    // this means it can flip the impacted cards of the impacted card (ignoring repeat plus/same rules)
    // this process continues recursively until there are no flips
    // we can implement this by calling card_impact on plus/same flipped cards with the combo flag
    pub fn check_pairs(
        &self,
        sides: [Option<ImpactPair>; 4],
        check_fn: fn(Vec<ImpactPair>) -> Option<Vec<usize>>,
    ) -> Vec<usize> {
        // println!("sides {:?}", sides);

        if let [Some(top), Some(right), Some(bottom), Some(left)] = sides {
            //   T
            // L   R
            //   B

            if let Some(flips) = check_fn(vec![top, right, bottom, left]) {
                if self.is_valid_plus_same(&flips) {
                    return flips;
                }
            }
        }

        if let [_, Some(right), Some(bottom), Some(left)] = sides {
            //   -
            // L   R
            //   B

            if let Some(flips) = check_fn(vec![right, bottom, left]) {
                if self.is_valid_plus_same(&flips) {
                    return flips;
                }
            }
        }

        if let [Some(top), _, Some(bottom), Some(left)] = sides {
            //   T
            // L   -
            //   B

            if let Some(flips) = check_fn(vec![top, bottom, left]) {
                if self.is_valid_plus_same(&flips) {
                    return flips;
                }
            }
        }

        if let [Some(top), Some(right), _, Some(left)] = sides {
            //   T
            // L   R
            //   -

            if let Some(flips) = check_fn(vec![top, right, left]) {
                if self.is_valid_plus_same(&flips) {
                    return flips;
                }
            }
        }

        if let [Some(top), Some(right), Some(bottom), _] = sides {
            //   T
            // -   R
            //   B

            if let Some(flips) = check_fn(vec![top, right, bottom]) {
                if self.is_valid_plus_same(&flips) {
                    return flips;
                }
            }
        }

        // if we didn't have a pure 4 match or a 3 match then
        // we could have 1 or 2 pairs here so we must check them all
        //
        // example with two pairs
        //
        //     2
        //     1
        // 1 2   2 3
        //     3
        //     2
        //
        // T + L form plus of 3
        // R + B form plus of 5

        let mut pair_flips = vec![];

        if let [Some(top), Some(right), _, _] = sides {
            //   T
            // -   R
            //   -

            if let Some(mut flips) = check_fn(vec![top, right]) {
                pair_flips.append(&mut flips);
            }
        }

        if let [Some(top), _, Some(bottom), _] = sides {
            //   T
            // -   -
            //   B

            if let Some(mut flips) = check_fn(vec![top, bottom]) {
                pair_flips.append(&mut flips);
            }
        }

        if let [Some(top), _, _, Some(left)] = sides {
            //   T
            // L   -
            //   -

            if let Some(mut flips) = check_fn(vec![top, left]) {
                pair_flips.append(&mut flips);
            }
        }

        if let [_, Some(right), Some(bottom), _] = sides {
            //   -
            // -   R
            //   B

            if let Some(mut flips) = check_fn(vec![right, bottom]) {
                pair_flips.append(&mut flips);
            }
        }

        if let [_, Some(right), _, Some(left)] = sides {
            //   -
            // L   R
            //   -

            if let Some(mut flips) = check_fn(vec![right, left]) {
                pair_flips.append(&mut flips);
            }
        }

        if let [_, _, Some(bottom), Some(left)] = sides {
            //   -
            // L   -
            //   B

            if let Some(mut flips) = check_fn(vec![bottom, left]) {
                pair_flips.append(&mut flips);
            }
        }

        // println!("pair_flips {:?}", pair_flips);

        if self.is_valid_plus_same(&pair_flips) {
            return pair_flips;
        }

        return vec![];
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

    pub fn percent_score(&self) -> f64 {
        return 100.0 * (self.score as f64 / self.turn as f64);
    }

    pub fn is_win(&self) -> bool {
        let is_score_over_half = self.score as f32 >= BOARD_SIZE as f32 / 2.0;
        return self.is_ended() && is_score_over_half;
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

    pub fn is_last_move(&self, index: usize) -> bool {
        if self.last_move < 0 {
            return false;
        }

        return self.last_move as usize == index;
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
                Card::player("P0", 2, 2, 2, 2),
                Card::player("P1", 2, 2, 2, 2),
                Card::player("P2", 2, 2, 2, 2),
                Card::player("P3", 2, 2, 2, 2),
                Card::player("P4", 2, 2, 2, 2),
            ],
        };

        let computer = Player {
            is_player: false,
            cards: [
                Card::computer("C0", 1, 1, 1, 1),
                Card::computer("C1", 1, 1, 1, 1),
                Card::computer("C2", 1, 1, 1, 1),
                Card::computer("C3", 1, 1, 1, 1),
                Card::computer("C4", 1, 1, 1, 1),
            ],
        };

        let rules = Rules {
            plus: false,
            same: false,
        };

        return Game {
            turn: 0,
            is_player_first: true,
            score: 0,
            rules,
            board,
            last_move: -1,

            player,
            computer,
        };
    }
}

pub fn same_pairs(pairs: Vec<ImpactPair>) -> Option<Vec<usize>> {
    match pairs.len() {
        4 => {
            let a = pairs[0];
            let b = pairs[1];
            let c = pairs[2];
            let d = pairs[3];

            if a.same && b.same && c.same && d.same {
                return Some(vec![a.square, b.square, c.square, d.square]);
            }
        }
        3 => {
            let a = pairs[0];
            let b = pairs[1];
            let c = pairs[2];

            if a.same && b.same && c.same {
                return Some(vec![a.square, b.square, c.square]);
            }
        }
        2 => {
            let a = pairs[0];
            let b = pairs[1];

            if a.same && b.same {
                return Some(vec![a.square, b.square]);
            }
        }
        _ => panic!("unexpected same_pairs [{:?}]", pairs),
    }

    return None;
}

pub fn plus_pairs(pairs: Vec<ImpactPair>) -> Option<Vec<usize>> {
    match pairs.len() {
        4 => {
            let a = pairs[0];
            let b = pairs[1];
            let c = pairs[2];
            let d = pairs[3];

            if a.sum == b.sum && c.sum == d.sum && a.sum == d.sum {
                return Some(vec![a.square, b.square, c.square, d.square]);
            }
        }
        3 => {
            let a = pairs[0];
            let b = pairs[1];
            let c = pairs[2];

            if a.sum == b.sum && b.sum == c.sum {
                return Some(vec![a.square, b.square, c.square]);
            }
        }
        2 => {
            let a = pairs[0];
            let b = pairs[1];

            if a.sum == b.sum {
                return Some(vec![a.square, b.square]);
            }
        }
        _ => panic!("unexpected plus_pairs [{:?}]", pairs),
    }

    return None;
}
