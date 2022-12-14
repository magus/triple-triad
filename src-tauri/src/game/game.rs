use rayon::prelude::*;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::card;
use crate::card::Card;
use crate::data::RuleJson;
use crate::game::constants::BOARD_SIZE;
use crate::game::impact::{self, ImpactPair};
use crate::player::Computer;
use crate::player::Player;
use crate::time::Stopwatch;

type Board = [Card; BOARD_SIZE];

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Rules {
    pub roulette: bool,
    pub all_open: bool,
    pub three_open: bool,
    pub same: bool,
    pub sudden_death: bool,
    pub plus: bool,
    pub random: bool,
    pub order: bool,
    pub chaos: bool,
    // ❌ TODO
    pub reverse: bool,
    // ❌ TODO
    pub fallen_ace: bool,
    // ❌ TODO
    pub ascension: bool,
    // ❌ TODO
    pub descension: bool,
    pub swap: bool,
    pub draft: bool,
}

impl Rules {
    pub fn from(&mut self, rules: &Vec<RuleJson>) {
        self.roulette = false;
        self.all_open = false;
        self.three_open = false;
        self.same = false;
        self.sudden_death = false;
        self.plus = false;
        self.random = false;
        self.order = false;
        self.chaos = false;
        self.reverse = false;
        self.fallen_ace = false;
        self.ascension = false;
        self.descension = false;
        self.swap = false;
        self.draft = false;

        for rule in rules {
            if false {
            } else if rule.is_roulette() {
                self.roulette = true;
            } else if rule.is_all_open() {
                self.all_open = true;
            } else if rule.is_three_open() {
                self.three_open = true;
            } else if rule.is_same() {
                self.same = true;
            } else if rule.is_sudden_death() {
                self.sudden_death = true;
            } else if rule.is_plus() {
                self.plus = true;
            } else if rule.is_random() {
                self.random = true;
            } else if rule.is_order() {
                self.order = true;
            } else if rule.is_chaos() {
                self.chaos = true;
            } else if rule.is_reverse() {
                self.reverse = true;
            } else if rule.is_fallen_ace() {
                self.fallen_ace = true;
            } else if rule.is_ascension() {
                self.ascension = true;
            } else if rule.is_descension() {
                self.descension = true;
            } else if rule.is_swap() {
                self.swap = true;
            } else if rule.is_draft() {
                self.draft = true;
            }
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Game {
    pub turn: u8,
    pub is_player_first: bool,
    pub score: i8,
    pub rules: Rules,
    pub board: Board,
    pub last_move: Option<(usize, usize)>,
    pub chaos_card: Option<usize>,
    pub evaluation_max: u64,

    pub player: Player,
    pub computer: Computer,
}

const DEFAULT_EVALUATION_MAX: u64 = 500_000_000;

#[derive(Clone, serde::Serialize)]
pub struct ExploreResult {
    pub total_depth_moves: u64,
    pub actual_moves_evaluated: u64,
    pub is_estimate: bool,
    pub results: Vec<ExploreResultItem>,
}

#[derive(Clone, serde::Serialize)]
pub struct ExploreResultItem {
    pub score: f64,
    pub game: Game,
}

impl Game {
    pub fn reset_evaluation_max(&mut self) {
        self.evaluation_max = DEFAULT_EVALUATION_MAX;
    }

    pub fn evaluate_explore_result(&self) -> ExploreResult {
        let mut stopwatch = Stopwatch::start();

        let start_turn = self.turn;
        let depth = 0;

        if self.is_ended() {
            println!("❌ Game is ended.");
            return ExploreResult {
                total_depth_moves: 0,
                actual_moves_evaluated: 0,
                is_estimate: false,
                results: vec![],
            };
        }

        let turn_is_player = self.turn_is_player();
        if self.turn == BOARD_SIZE as u8 - 1 && turn_is_player {
            println!("❌ There is only one possible move.");

            let square_choices = self.squares_empty();
            let card_choices = self.player.cards_left(None, self.rules.order);

            if card_choices.len() > 1 || square_choices.len() > 1 {
                panic!("🚨 Invariant state, unexpected more than one possible move");
            }

            let game = self.clone();
            let card_index = *card_choices.first().unwrap();
            let square_index = *square_choices.first().unwrap();
            let game = game.execute_turn(card_index, square_index);
            let score = if game.is_win() { 100.0 } else { 0.0 };

            return ExploreResult {
                total_depth_moves: 1,
                actual_moves_evaluated: 1,
                is_estimate: false,
                results: vec![ExploreResultItem { score, game }],
            };
        }

        let total_depth_moves = self.max_depth_moves(self.turn, -1);
        println!("🔍 Evaluting [{total_depth_moves}] possible game states ... ");

        // on turn 1 when player is first the search space is worst case scenario
        // so we search for a safe max_depth to signficantly cut search space
        //
        //     = 36,578,304,000 paths
        //     = 45 * 56 * 28 * 36 * 15 * 20 * 6 * 8 * 1
        //
        // once we have heuristics use it to cutoff instead of depth
        // at max_depth we can instead use the score of a game to decide
        // whether to recurse with game.explore, again culling search space
        let mut max_depth = BOARD_SIZE as i8;

        for test_depth in (1..max_depth).rev() {
            max_depth = test_depth;

            let depth_moves = self.max_depth_moves(self.turn, max_depth);
            if depth_moves < self.evaluation_max {
                println!("✅ Found safe depth [{max_depth}] with [{depth_moves}] moves!");
                break;
            }
        }

        println!();
        println!("🤖 ... thinking ...");
        println!();

        // collect results across threads into shard vector
        let results: Arc<Mutex<Vec<(f64, Game)>>> = Arc::new(Mutex::new(Vec::new()));

        self.explore(start_turn, max_depth, depth, &results);

        // sort results by comparing score values
        let mut explore_result_list = results.lock().unwrap().deref().clone();
        explore_result_list.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let max_depth_moves = self.max_depth_moves(self.turn + 1, max_depth);

        let mut results = vec![];

        for explore_result in explore_result_list {
            let (total_score, game) = explore_result;
            let score = total_score / max_depth_moves as f64;

            results.push(ExploreResultItem { game, score })
        }

        let total_depth_moves = self.max_depth_moves(self.turn, -1);
        let actual_moves_evaluated = self.max_depth_moves(self.turn, max_depth);
        let is_estimate = total_depth_moves > actual_moves_evaluated;

        stopwatch.record("evaluate_explore_result");

        return ExploreResult {
            total_depth_moves,
            actual_moves_evaluated,
            is_estimate,
            results,
        };
    }

    pub fn print_explore(&self) {
        let mut stopwatch = Stopwatch::start();

        let explore_result = self.evaluate_explore_result();
        let results = explore_result.results;

        println!("┌─────────────────────────────────────────┐");
        println!("│  🤖 AI RECOMMENDATIONS (worst to best)  │");
        println!("└─────────────────────────────────────────┘");
        println!();

        // show up to top 3 moves
        let show_count = std::cmp::min(3, results.len());

        for i in 0..show_count {
            let index = results.len() - 1 - i;
            let result = &results[index];

            println!("#{}", index + 1);
            println!("{:.2}% chance to win", result.score);
            println!();
            println!("{:?}", result.game);
        }

        println!("...\n\n");

        for i in (0..show_count).rev() {
            let result = &results[i];

            println!("#{}", i + 1);
            println!("{:.2}% chance to win", result.score);
            println!();
            println!("{:?}", result.game);
        }

        let total_depth_moves = explore_result.total_depth_moves;
        let actual_moves_evaluated = explore_result.actual_moves_evaluated;

        if explore_result.is_estimate {
            println!("✅ done [📊 {actual_moves_evaluated} / {total_depth_moves} moves evaluated]");
        } else {
            println!("✅ done [📊 {actual_moves_evaluated} moves evaluated]");
        }

        stopwatch.record("start_explore");
    }

    fn explore(
        &self,
        start_turn: u8,
        max_depth: i8,
        depth: u8,
        results: &Arc<Mutex<Vec<(f64, Game)>>>,
    ) -> f64 {
        // println!("max_depth={max_depth}, start_turn={start_turn}, depth={depth}");

        let next_depth = depth + 1;

        // find all valid moves from this game state and execute them
        let is_player = self.turn_is_player();
        let square_choices = self.squares_empty();
        let card_choices = if is_player {
            self.player.cards_left(self.chaos_card, self.rules.order)
        } else {
            self.computer.cards_left()
        };

        let total_moves = square_choices.par_iter().map(|square| {
            let total_square = card_choices.par_iter().map(|card| {
                let game = self.execute_turn(*card, *square);

                if game.is_ended() {
                    // pass win-loss back as score
                    return if game.is_win() { 100.0 } else { 0.0 };
                } else if next_depth as i8 == max_depth + 1 {
                    // println!(
                    //     "turn={} depth={depth} next_depth={next_depth}, max_depth={max_depth}",
                    //     game.turn
                    // );

                    // TODO we can use other metrics here to score unfinished games
                    // count sides which can be flipped by a remaining card of opponent
                    // solid sides which cannot be flipped are valuable

                    // // debug by returning perfect score so results should all show 100%
                    // return 100.0;

                    // return self.percent_score();

                    // evaluating score to determine boolean win shot
                    return if self.is_winning() { 100.0 } else { 0.0 };
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
        let target_depth = start_turn + 1;
        if self.turn == target_depth {
            results.lock().unwrap().push((total_score, self.clone()));
        }

        return total_score;
    }

    pub fn execute_turn(&self, card_index: usize, square_index: usize) -> Game {
        let mut game = self.clone();

        if game.place_card(card_index, square_index) {
            game.finish_turn();
        }

        return game;
    }

    pub fn simulate_simple_turn(&self) -> Game {
        // println!();
        // println!();

        if self.is_ended() {
            // println!("  [game ended]");
            return self.clone();
        }

        // println!("[start turn {}] {}", self.turn + 1, self.player_name());
        // println!("  [player.cards_left] {:?}", self.player.cards_left());
        // println!("  [player] [{:?}]", self.player.cards);
        // println!("  [computer.cards_left] {:?}", self.computer.cards_left());
        // println!("  [computer] [{:?}]", self.computer.cards);
        // println!("  [squares_empty] {:?}", self.squares_empty());
        // println!("  [board] {:?}", self.board);

        // find first valid move and make it
        let square_choices = self.squares_empty();
        let is_player = self.turn_is_player();

        let card_choices = if is_player {
            self.player.cards_left(self.chaos_card, self.rules.order)
        } else {
            self.computer.cards_left()
        };

        let game = self.execute_turn(card_choices[0], square_choices[0]);

        // println!("[end turn {}]", game.turn);
        // println!("  [player.cards_left] {:?}", game.player.cards_left());
        // println!("  [player] [{:?}]", game.player.cards);
        // println!("  [computer.cards_left] {:?}", game.computer.cards_left());
        // println!("  [computer] [{:?}]", game.computer.cards);
        // println!("  [squares_empty] {:?}", game.squares_empty());
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

    pub fn place_card(&mut self, card_index: usize, index: usize) -> bool {
        if self.board[index] == card::EMPTY {
            let is_player = self.turn_is_player();

            // get the card from the correct players hand
            let card = if is_player {
                self.player.use_card(card_index)
            } else {
                self.computer.use_card(card_index)
            };

            if card != card::EMPTY {
                // place the card in this board square
                self.board[index] = card;

                let is_combo = false;
                self.card_impact(index, is_combo);
                self.last_move = Some((card_index, index));

                self.handle_tribe(&card);

                // now that card is placed, can we guarantee computer cards?
                if !is_player {
                    self.computer.maybe_filter_hand_guaranteed();
                }

                return true;
            }
        }

        // if we got here it means we failed for some reason
        println!("❌ Invalid card choice");
        return false;
    }

    fn handle_tribe(&mut self, card: &Card) {
        // no tribe rules, skip
        if !self.rules.ascension && !self.rules.descension {
            return;
        }

        // no tribe, skip
        if card.tribe == 0 {
            return;
        }

        let modifier = if self.rules.ascension {
            1
        } else if self.rules.descension {
            -1
        } else {
            panic!("unexpected scenario neither ascension or descension but in else block");
        };

        // increment cards on board
        for i in 0..BOARD_SIZE {
            let square = &self.board[i];

            if !square.is_empty && square.tribe == card.tribe {
                self.board[i].modifier += modifier;
            }
        }

        // increment cards in hands
        for i in 0..self.player.cards.len() {
            let hand_card = &self.player.cards[i];

            if !hand_card.is_empty && hand_card.tribe == card.tribe {
                self.player.cards[i].modifier += modifier;
            }
        }
        for i in 0..self.computer.cards.len() {
            let hand_card = &self.computer.cards[i];

            if !hand_card.is_empty && hand_card.tribe == card.tribe {
                self.computer.cards[i].modifier += modifier;
            }
        }
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

        let fallen_ace;
        let normal;

        // fallen ace
        // 1 flips A; reverse A flips 1
        if self.rules.reverse {
            fallen_ace = placed == 10 && impacted == 1;
            normal = placed < impacted;
        } else {
            fallen_ace = placed == 1 && impacted == 10;
            normal = placed > impacted;
        }

        return fallen_ace || normal;
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
        // clear potential chaos_card
        self.chaos_card = None;
        // increment turn counter driving game forward
        self.turn += 1;
    }

    pub fn execute_swap(&mut self, player_card_index: usize, computer_card_index: usize) {
        let mut computer_card = self.computer.cards[computer_card_index];
        computer_card.is_player = true;

        let mut player_card = self.player.cards[player_card_index];
        player_card.is_player = false;
        // always mark guaranteed since we know it's there since we swapped it in
        player_card.is_guaranteed = true;

        self.player.cards[player_card_index] = computer_card;
        self.computer.cards[computer_card_index] = player_card;
    }

    pub fn percent_score(&self) -> f64 {
        return 100.0 * (self.score as f64 / BOARD_SIZE as f64);
    }

    pub fn is_winning(&self) -> bool {
        let draw_score = (BOARD_SIZE as f32 / 2.0).ceil() as i8;
        return self.score > draw_score;
    }

    pub fn is_win(&self) -> bool {
        return self.is_ended() && self.is_winning();
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
        if let Some((_, square_index)) = self.last_move {
            return square_index == index;
        }

        return false;
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
            cards: [
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
            ],
        };

        let computer = Computer {
            cards_used: 0,
            cards: [
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
                card::EMPTY,
            ],
        };

        let rules = Rules {
            roulette: false,
            all_open: false,
            three_open: false,
            same: false,
            sudden_death: false,
            plus: false,
            random: false,
            order: false,
            chaos: false,
            reverse: false,
            fallen_ace: false,
            ascension: false,
            descension: false,
            swap: false,
            draft: false,
        };

        return Game {
            turn: 0,
            is_player_first: true,
            score: 5,
            rules,
            board,
            last_move: None,
            chaos_card: None,
            evaluation_max: 500_000_000,

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
