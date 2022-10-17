use colored::*;

use crate::card;
use crate::card::Card;
use crate::data;
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
    let game = setup_game(&npc_data, &rule_data);

    // Handle things like swap, reveal, etc.
    let game = post_setup_game(game);
    println!("{:?}", game);

    // Then alternate inputting in moves
    // On each play step print game board + both player cards
    drive_game(game);
}

fn drive_game(input_game: Game) {
    let mut game = input_game.clone();

    print_drive_game_help();

    loop {
        let input = print::prompt();

        // close is to capture returns allowing us to break out of match
        // in order to rerun our outer control prompt loop
        (|| match input.as_str() {
            "p" | "put" => {
                game.print_turn_hand();
                println!();
                println!("{}", print::box_text("Which card?", 1));
                let is_player = game.turn_is_player();
                let maybe_card = prompt_card_index(&mut game, is_player);

                if maybe_card == None {
                    return;
                }

                game.print_board();
                println!();
                println!("{}", print::box_text("Which square?", 1));
                let maybe_square = print::prompt().parse::<usize>();
                if let Ok(square) = maybe_square {
                    if square > constants::BOARD_SIZE {
                        println!("❌ square must be between 0 and {}", constants::BOARD_SIZE);
                        return;
                    }
                } else {
                    println!("❌ square must be a positive number");
                    return;
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
                    let maybe_card = prompt_card_index(&mut game, true);

                    if let Some(card) = maybe_card {
                        game.chaos_card = Some(card);
                    } else {
                        return;
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
                panic!("❌ aborting game")
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
        })()
    }
}

fn setup_game(npc_data: &NpcData, rule_data: &RuleData) -> Game {
    let mut game = Game::new();

    loop {
        print_setup_help();

        let input = print::prompt();

        match input.as_str() {
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

    // manually setting up cards for now
    // todo handle setting up player and computer via commands above

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3),
        Card::player("P1", 8, 2, 3, 8),
        Card::player("P2", 1, 8, 3, 8),
        Card::player("P3", 1, 5, 9, 9),
        Card::player("P4", 6, 10, 10, 1),
    ];

    println!("{}", print::box_text("✅ Setup complete!", 1));

    return game;
}

fn post_setup_game(input_game: Game) -> Game {
    let mut game = input_game.clone();

    // handle swap
    if game.rules.swap {
        println!("🔀 Swap");
        loop {
            println!("{}", print::box_text("Which card was taken from you?", 1));
            game.print_player_hand();
            let maybe_player_card = prompt_card_index(&mut game, true);
            if maybe_player_card == None {
                continue;
            }
            println!("{}", print::box_text("Which card was given to you?", 1));
            game.print_computer_hand();
            let maybe_computer_card = prompt_card_index(&mut game, false);
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

fn print_setup_help() {
    println!();

    println!(
        "{}\tsearch {} by name and load",
        "(n)npc".white().bold(),
        "npc".white().bold(),
    );
    println!(
        "{}\ttoggle the {} player",
        "(f)irst".white().bold(),
        "first".white().bold(),
    );
    println!(
        "{}\tselect {} to toggle from list",
        "(r)ules".white().bold(),
        "rules".white().bold(),
    );
    println!(
        "{}\tsetup for game is {}, ready to play",
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
    println!(
        "{}\t\t{}matically play best AI recommended move",
        "(a)uto".white().bold(),
        "auto".white().bold(),
    );
    println!(
        "{}\t\tbegin {}ing a card on the board",
        "(p)ut".white().bold(),
        "put".white().bold(),
    );
    println!(
        "{}\t{} for 🤖 AI recommended moves",
        "(s)earch".white().bold(),
        "search".white().bold(),
    );
    println!(
        "{}\t\t{} the last move",
        "(u)ndo".white().bold(),
        "undo".white().bold(),
    );
    println!(
        "{}\t\t{} the last undo",
        "(r)edo".white().bold(),
        "redo".white().bold(),
    );
    println!();
    println!(
        "(enter {} to quit at any time)",
        "q".truecolor(197, 3, 3).bold()
    );
}

fn prompt_card_index(game: &mut Game, is_player: bool) -> Option<usize> {
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
