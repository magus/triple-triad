use crate::card;
use crate::card::Card;
use crate::game::constants;
use crate::game::Game;
use crate::print;
use colored::*;

pub fn drive_game_prompt() {
    // First phase sets up rules and first player
    // Then alternate inputting in moves
    // On each play step print game board + both player cards

    let mut game = setup_game();

    // https://arrtripletriad.com/en/npc-idle-imperial

    game.player.cards = [
        Card::player("P0", 8, 8, 2, 3),
        Card::player("P1", 8, 2, 3, 8),
        Card::player("P2", 1, 8, 3, 8),
        Card::player("P3", 1, 5, 9, 9),
        Card::player("P4", 6, 10, 10, 1),
    ];

    game.computer.cards = [
        Card::computer_guaranteed("C0", 6, 3, 7, 3),
        Card::computer_guaranteed("C1", 9, 7, 8, 1),
        Card::computer("C2", 4, 1, 8, 7),
        Card::computer("C3", 7, 1, 6, 7),
        Card::computer("C4", 1, 4, 8, 8),
        Card::computer("C5", 7, 1, 3, 7),
        Card::computer("C6", 8, 3, 8, 1),
        card::EMPTY,
    ];

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
                let maybe_card = print::prompt().parse::<usize>();
                if let Ok(card) = maybe_card {
                    let hand_max = game.turn_player_hand_max();

                    if card > hand_max {
                        println!("❌ card must be between 0 and {}", hand_max);
                        return;
                    }
                } else {
                    println!("❌ card must be a positive number");
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

                if let (Ok(card), Ok(square)) = (maybe_card, maybe_square) {
                    game = game.execute_turn(card, square);
                    println!("{:?}", game);
                }
            }
            "s" | "search" => {
                // handle chaos when search for moves with explore
                if game.turn_is_player() && game.rules.chaos {
                    game.print_turn_hand();
                    println!();
                    println!("{}", print::box_text("Which card did chaos select?", 1));
                    let maybe_card = print::prompt().parse::<usize>();
                    if let Ok(card) = maybe_card {
                        let hand_max = game.turn_player_hand_max();

                        if card > hand_max {
                            println!("❌ card must be between 0 and {}", hand_max);
                            return;
                        }

                        if game.player.cards[card] == card::EMPTY {
                            println!("❌ there is no card at {}", card);
                            return;
                        }

                        game.chaos_card = Some(card);
                    } else {
                        println!("❌ card must be a positive number");
                        return;
                    }
                }

                game.start_explore();
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

fn setup_game() -> Game {
    let mut game = Game::new();

    print_setup_help();

    loop {
        let input = print::prompt();

        match input.as_str() {
            "f" | "first" => {
                game.is_player_first = !game.is_player_first;
                println!("{}", game.print_rules());
            }
            "c" | "chaos" => {
                game.rules.chaos = !game.rules.chaos;
                println!("{}", game.print_rules());
            }
            "p" | "plus" => {
                game.rules.plus = !game.rules.plus;
                println!("{}", game.print_rules());
            }
            "s" | "same" => {
                game.rules.same = !game.rules.same;
                println!("{}", game.print_rules());
            }
            "d" | "done" => {
                println!("{}", game.print_rules());
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

fn print_setup_help() {
    println!(
        "{}\tToggle the {} player",
        "(f)irst".white().bold(),
        "first".white().bold(),
    );
    println!(
        "{}\tToggle the {} rule",
        "(c)haos".white().bold(),
        "chaos".white().bold(),
    );
    println!(
        "{}\tToggle the {} rule",
        "(p)lus".white().bold(),
        "plus".white().bold(),
    );
    println!(
        "{}\tToggle the {} rule",
        "(s)ame".white().bold(),
        "same".white().bold(),
    );
    println!(
        "{}\tSetup for game is {}, ready to play",
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
        "{}\t\tBegin {}ing a card on the board",
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
