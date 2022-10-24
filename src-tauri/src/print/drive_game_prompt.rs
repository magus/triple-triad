use colored::*;

use crate::card;
use crate::card::Card;
use crate::data;
use crate::data::CardData;
use crate::data::NpcData;
use crate::data::RuleData;
use crate::game::constants;
use crate::game::Game;
use crate::print;
use crate::time::Stopwatch;

pub fn drive_game_prompt() {
    let mut stopwatch = Stopwatch::start();

    let rule_data = data::RuleData::read();
    let card_data = data::CardData::read();
    let npc_data = data::NpcData::read(&card_data, &rule_data);

    stopwatch.record("drive_game_prompt load game data");

    // First phase sets up rules, first player, etc.
    let mut init_game = setup_game(&npc_data, &rule_data, &card_data);

    loop {
        // ensure fresh instance of game on each loop
        let game = init_game.clone();

        // Handle things like swap, reveal, etc.
        let game = pre_game(game, &card_data);
        println!("{:?}", game);

        // Then alternate inputting in moves
        // On each play step print game board + both player cards
        let game = drive_game(game);

        // Handle things like sudden death
        init_game = post_game(&game, &init_game);
    }
}

fn drive_game(input_game: Game) -> Game {
    let mut game = input_game.clone();

    print_drive_game_help();

    loop {
        if game.is_ended() {
            break;
        }

        let input = print::prompt();

        match input.as_str() {
            "p" | "put" => {
                game.print_turn_hand();
                println!();
                println!("{}", print::box_text("Which card?", 1));
                let is_player = game.turn_is_player();
                let maybe_card = prompt_card_index(&game, is_player);

                if maybe_card == None {
                    continue;
                }

                game.print_board();
                println!();
                println!("{}", print::box_text("Which square?", 1));
                let maybe_square = print::prompt().parse::<usize>();
                if let Ok(square) = maybe_square {
                    if square > constants::BOARD_SIZE {
                        println!("❌ square must be between 0 and {}", constants::BOARD_SIZE);
                        continue;
                    }
                } else {
                    println!("❌ square must be a positive number");
                    continue;
                }

                if let (Some(card), Ok(square)) = (maybe_card, maybe_square) {
                    game = game.execute_turn(card, square);
                    println!("{:?}", game);
                }
            }
            "a" | "auto" | "s" | "search" | "e" | "max" | "evaluation max" => {
                // handle chaos when search for moves with explore
                if game.turn_is_player() && game.rules.chaos {
                    game.print_turn_hand();
                    println!();
                    println!("{}", print::box_text("Which card did chaos select?", 1));
                    let maybe_card = prompt_card_index(&game, true);

                    if let Some(card) = maybe_card {
                        game.chaos_card = Some(card);
                    } else {
                        continue;
                    }
                }

                // handle evaluation max set
                match input.as_str() {
                    "e" | "max" | "evaluation max" => {
                        println!();
                        println!("{}", print::box_text("Evaluation Max", 1));
                        let maybe_evaluation_max = print::prompt().parse::<u64>();

                        if let Err(_) = maybe_evaluation_max {
                            println!("❌ invalid max must be between 1 and {}", u64::MAX);
                        } else {
                            game.evaluation_max = maybe_evaluation_max.unwrap();
                        }
                    }
                    _ => {
                        // do nothing
                    }
                }

                let moves = game.start_explore();

                match input.as_str() {
                    "a" | "auto" => {
                        if let Some((_, move_game)) = moves.first() {
                            if let Some((card_index, square_index)) = move_game.last_move {
                                game = game.execute_turn(card_index, square_index);
                            }
                        }
                    }
                    // "s" | "search" => {}
                    _ => {
                        // do nothing just explore
                    }
                }

                game.reset_evaluation_max();
            }
            "u" | "undo" => {
                println!("TODO UNDO");
            }
            "r" | "redo" => {
                println!("TODO REDO");
            }
            "q" | "quit" => {
                println!("❌ restarting game");
                return game;
            }
            "h" | "help" => {
                print_drive_game_help();
            }
            "" => {}
            _ => println!(
                "❌ [{}] (unrecognized command, type {} for commands)",
                input,
                "(h)elp".white().bold()
            ),
        }
    }

    return game;
}

fn setup_game(npc_data: &NpcData, rule_data: &RuleData, card_data: &CardData) -> Game {
    let mut game = Game::new();

    loop {
        print_setup_help();

        let input = print::prompt();

        match input.as_str() {
            "p" | "player" => {
                game = select_player_cards(&game, &card_data);
            }
            "n" | "npc" => {
                // close is to capture returns allowing us to break out of match
                // in order to rerun our outer control prompt loop
                let maybe_npc = (|| {
                    loop {
                        println!();
                        println!("{}", print::box_text("🔍 Search for NPC", 1));
                        let maybe_search = print::prompt().parse::<String>();
                        if let Err(_) = maybe_search {
                            // this should never happen but just handle err case
                            // so we can unwrap below, to be super explicit
                            println!("❌ invalid search input");
                            continue;
                        }

                        let search = maybe_search.unwrap();
                        let results = npc_data.search(&search);
                        match results.len() {
                            0 => {
                                println!("❌ no results found");
                                continue;
                            }
                            1 => {
                                // exact match, proceed with npc
                                let npc = results.first().unwrap().clone();
                                return Some(npc);
                            }
                            _ => {
                                println!("{}", print::box_text("Which NPC?", 1));

                                for i in 0..results.len() {
                                    let npc = results[i];
                                    println!("[{i}] {}", npc.name);
                                }

                                let maybe_i = print::prompt().parse::<usize>();

                                if let Err(_) = maybe_i {
                                    println!("❌ invalid selection");
                                    continue;
                                }

                                let i = maybe_i.unwrap();

                                if !(i < results.len()) {
                                    println!("❌ invalid selection");
                                    continue;
                                }

                                let npc = results[i];
                                return Some(npc);
                            }
                        }
                    }
                })();

                let npc = maybe_npc.unwrap();
                println!("{:#?}", npc);

                // use the npc to setup game
                game = Game::new();
                game.computer.cards_from(npc.cards.clone());
                game.print_computer_hand();
                game.rules.from(&npc.rules);
                println!("{}", game.print_rules(rule_data));
            }
            "f" | "first" => {
                game.is_player_first = !game.is_player_first;
                println!("{}", game.print_rules(rule_data));
            }
            "r" | "rules" => {
                loop {
                    println!();

                    println!("{}", game.print_rules(rule_data));
                    println!();
                    println!(
                        "(enter {} to finish editing rules)",
                        "d".truecolor(197, 3, 3).bold()
                    );

                    let maybe_id = print::prompt().parse::<String>();

                    if let Ok(input) = maybe_id.clone() {
                        match input.as_str() {
                            "d" | "done" | "q" | "quit" => {
                                break;
                            }
                            _ => {}
                        }
                    }

                    if let Err(_) = maybe_id {
                        println!("❌ invalid selection");
                        continue;
                    }

                    let id = maybe_id.unwrap();
                    let maybe_rule = rule_data.by_id(&id);

                    if let Some(_) = maybe_rule {
                    } else {
                        println!("❌ invalid selection");
                        continue;
                    }

                    let rule = maybe_rule.unwrap();

                    if false {
                    } else if rule.is_roulette() {
                        game.rules.roulette = !game.rules.roulette;
                    } else if rule.is_all_open() {
                        game.rules.all_open = !game.rules.all_open;
                    } else if rule.is_three_open() {
                        game.rules.three_open = !game.rules.three_open;
                    } else if rule.is_same() {
                        game.rules.same = !game.rules.same;
                    } else if rule.is_sudden_death() {
                        game.rules.sudden_death = !game.rules.sudden_death;
                    } else if rule.is_plus() {
                        game.rules.plus = !game.rules.plus;
                    } else if rule.is_random() {
                        game.rules.random = !game.rules.random;
                    } else if rule.is_order() {
                        game.rules.order = !game.rules.order;
                    } else if rule.is_chaos() {
                        game.rules.chaos = !game.rules.chaos;
                    } else if rule.is_reverse() {
                        game.rules.reverse = !game.rules.reverse;
                    } else if rule.is_fallen_ace() {
                        game.rules.fallen_ace = !game.rules.fallen_ace;
                    } else if rule.is_ascension() {
                        game.rules.ascension = !game.rules.ascension;
                    } else if rule.is_descension() {
                        game.rules.descension = !game.rules.descension;
                    } else if rule.is_swap() {
                        game.rules.swap = !game.rules.swap;
                    } else if rule.is_draft() {
                        game.rules.draft = !game.rules.draft;
                    }
                }

                println!("{}", game.print_rules(rule_data));
            }
            "d" | "done" => {
                println!("{}", game.print_rules(rule_data));
                break;
            }
            "q" | "quit" => {
                panic!("❌ aborting setup")
            }
            "h" | "help" => {
                print_setup_help();
            }
            "" => {}
            _ => println!(
                "❌ [{}] (unrecognized command, type {} for commands)",
                input,
                "(h)elp".white().bold()
            ),
        }
    }

    println!("{}", print::box_text("✅ Setup complete!", 1));

    return game;
}

fn pre_game(input_game: Game, card_data: &CardData) -> Game {
    let mut game = input_game.clone();

    if game.rules.draft {
        println!("🎲 Draft");
        game = select_player_cards(&game, &card_data);
    } else if game.rules.random {
        println!("🎲 Random");
        game = select_player_cards(&game, &card_data);
    }

    if game.rules.all_open {
        println!("👀 All Open");
        let mut cards = vec![];
        let mut temp_game = game.clone();
        let required_cards: usize = 5;

        loop {
            let message = format!(
                "Which cards are revealed? [{} / {required_cards}]",
                cards.len() + 1
            );
            println!("{}", print::box_text(&message, 1));

            temp_game.print_computer_hand();
            let maybe_computer_card = prompt_card_index(&temp_game, false);
            if maybe_computer_card == None {
                continue;
            }

            let computer_card = maybe_computer_card.unwrap();
            cards.push(temp_game.computer.cards[computer_card]);
            temp_game.computer.cards[computer_card] = card::EMPTY;

            if cards.len() == required_cards {
                break;
            }
        }

        game.computer.cards_from(cards);
    } else if game.rules.three_open {
        println!("👀 Three Open");
        let mut cards = vec![];
        let mut temp_game = game.clone();
        let required_cards: usize = 3;

        loop {
            let message = format!(
                "Which cards are revealed? [{} / {required_cards}]",
                cards.len() + 1
            );
            println!("{}", print::box_text(&message, 1));

            temp_game.print_computer_hand();
            let maybe_computer_card = prompt_card_index(&temp_game, false);
            if maybe_computer_card == None {
                continue;
            }

            let computer_card = maybe_computer_card.unwrap();
            cards.push(computer_card);
            temp_game.computer.cards[computer_card] = card::EMPTY;

            if cards.len() == required_cards {
                break;
            }
        }

        for card_index in cards {
            game.computer.cards[card_index].is_guaranteed = true;
        }
    }

    if game.rules.swap {
        println!("🔀 Swap");
        loop {
            println!("{}", print::box_text("Which card was taken from you?", 1));
            game.print_player_hand();
            let maybe_player_card = prompt_card_index(&game, true);
            if maybe_player_card == None {
                continue;
            }
            println!("{}", print::box_text("Which card was given to you?", 1));
            game.print_computer_hand();
            let maybe_computer_card = prompt_card_index(&game, false);
            if maybe_computer_card == None {
                continue;
            }

            if let (Some(player_card), Some(computer_card)) =
                (maybe_player_card, maybe_computer_card)
            {
                println!(
                    "🔀 Exchanging [{player_card}] (player) with [{computer_card}] from (computer)"
                );
                println!();

                game.execute_swap(player_card, computer_card);

                // exit loop
                break;
            }
        }
    }

    return game;
}

fn post_game(game: &Game, init_game: &Game) -> Game {
    let game = game.clone();

    if !game.is_ended() {
        return init_game.clone();
    }

    if game.rules.sudden_death && game.score == 5 {
        println!("☠️ Sudden Death");

        let mut player_cards = vec![];
        let mut computer_cards = vec![];

        for card in game.board {
            if card.is_player {
                player_cards.push(card);
            } else {
                // mark card guaranteed to optimize next run
                let mut edit_card = card.clone();
                edit_card.is_guaranteed = true;
                computer_cards.push(edit_card);
            }
        }

        for card in game.player.cards {
            if card != card::EMPTY {
                player_cards.push(card);
            }
        }

        for card in game.computer.cards {
            if card != card::EMPTY {
                computer_cards.push(card);
            }
        }

        let mut new_init_game = init_game.clone();
        new_init_game.player.cards_from(player_cards);
        new_init_game.computer.cards_from(computer_cards);
        return new_init_game;
    }

    return init_game.clone();
}

fn print_setup_help() {
    let cmd = 10;

    println!();

    println!(
        "{:<cmd$} select {} cards by name",
        "(p)layer".white().bold(),
        "player".white().bold(),
    );
    println!(
        "{:<cmd$} select {} by name and load",
        "(n)npc".white().bold(),
        "npc".white().bold(),
    );
    println!(
        "{:<cmd$} toggle the {} player",
        "(f)irst".white().bold(),
        "first".white().bold(),
    );
    println!(
        "{:<cmd$} select {} to toggle from list",
        "(r)ules".white().bold(),
        "rules".white().bold(),
    );
    println!(
        "{:<cmd$} setup for game is {}, ready to play",
        "(d)one".white().bold(),
        "done".white().bold(),
    );
    println!();
    println!(
        "(enter {} to quit at any time)",
        "q".truecolor(197, 3, 3).bold()
    );
}

fn print_drive_game_help() {
    let cmd = 18;

    println!();

    println!(
        "{:<cmd$} {}matically play best AI recommended move",
        "(a)uto".white().bold(),
        "auto".white().bold(),
    );
    println!(
        "{:<cmd$} begin {}ing a card on the board",
        "(p)ut".white().bold(),
        "put".white().bold(),
    );
    println!(
        "{:<cmd$} {} for 🤖 AI recommended moves",
        "(s)earch".white().bold(),
        "search".white().bold(),
    );
    println!(
        "{:<cmd$} set {} for 🤖 AI recommended moves",
        "(e)valuation max".white().bold(),
        "evaluation max".white().bold(),
    );
    println!(
        "{:<cmd$} {} the last move",
        "(u)ndo".white().bold(),
        "undo".white().bold(),
    );
    println!(
        "{:<cmd$} {} the last undo",
        "(r)edo".white().bold(),
        "redo".white().bold(),
    );
    println!();
    println!(
        "(enter {} to quit at any time)",
        "q".truecolor(197, 3, 3).bold()
    );
}

fn prompt_card_index(game: &Game, is_player: bool) -> Option<usize> {
    let maybe_card = print::prompt().parse::<usize>();

    if let Ok(card) = maybe_card {
        let hand_max = if is_player {
            game.player.cards.len() - 1
        } else {
            game.computer.cards.len() - 1
        };

        if card > hand_max {
            println!("❌ card must be between 0 and {}", hand_max);
            return None;
        }

        let is_invalid = if is_player {
            game.player.cards[card] == card::EMPTY
        } else {
            game.computer.cards[card] == card::EMPTY
        };

        if is_invalid {
            println!("❌ there is no card at {}", card);
            return None;
        }

        return Some(card);
    } else {
        println!("❌ card must be a positive number");
        return None;
    }
}

fn select_player_cards(game: &Game, card_data: &CardData) -> Game {
    let mut game = game.clone();

    let mut cards = vec![];

    let required_cards: usize = 5;

    // sync cards back to game
    game.player.cards_from(cards.clone());

    loop {
        game.print_player_hand();

        let message = format!(
            "🔍 Search for Card [{} / {required_cards}]",
            cards.len() + 1
        );
        println!("{}", print::box_text(&message, 1));

        let maybe_search = print::prompt().parse::<String>();
        if let Err(_) = maybe_search {
            // this should never happen but just handle err case
            // so we can unwrap below, to be super explicit
            println!("❌ invalid search input");
            continue;
        }

        let search = maybe_search.unwrap();
        let results = card_data.search(&search);
        match results.len() {
            0 => {
                println!("❌ no results found");
                continue;
            }
            1 => {
                // exact match, proceed with card
                let card = results.first().unwrap().clone();

                cards.push(Card::player(
                    Card::player_name(cards.len()),
                    card.top,
                    card.right,
                    card.bottom,
                    card.left,
                    card.tribe,
                ));
            }
            _ => {
                println!("{}", print::box_text("Which Card?", 1));

                for i in 0..results.len() {
                    let result = results[i];
                    println!("[{i}] {}", result.name);
                }

                let maybe_i = print::prompt().parse::<usize>();

                if let Err(_) = maybe_i {
                    println!("❌ invalid selection");
                    continue;
                }

                let i = maybe_i.unwrap();

                if !(i < results.len()) {
                    println!("❌ invalid selection");
                    continue;
                }

                let card = results[i];
                cards.push(Card::player(
                    Card::player_name(cards.len()),
                    card.top,
                    card.right,
                    card.bottom,
                    card.left,
                    card.tribe,
                ));
            }
        }

        // sync cards back to game
        game.player.cards_from(cards.clone());

        if cards.len() == required_cards {
            return game;
        }
    }
}
