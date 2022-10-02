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
    pub fn execute_turn(&self) -> Game {
        let mut game = self.clone();
        let is_player = game.turn_is_player();

        println!("");
        println!("");
        println!("[start turn {}] {}", game.turn + 1, game.player_name());
        println!("[player.cards_left] {:?}", game.player.cards_left());
        // println!("[player] [{:?}]", game.player.cards);
        println!("[computer.cards_left] {:?}", game.computer.cards_left());
        // println!("[computer] [{:?}]", game.computer.cards);
        println!("[squares_empty] {:?}", game.squares_empty());
        // println!("[board] {:?}", game.board);

        if is_player {
            let card = game.player.find_card();
            game.find_place_card(card);
        } else {
            let card = game.computer.find_card();
            game.find_place_card(card);
        };

        println!("[player.cards_left] {:?}", game.player.cards_left());
        // println!("[player] [{:?}]", game.player.cards);
        println!("[computer.cards_left] {:?}", game.computer.cards_left());
        // println!("[computer] [{:?}]", game.computer.cards);
        println!("[squares_empty] {:?}", game.squares_empty());
        // println!("[board] {:?}", game.board);
        println!("[end turn {}] {}", game.turn + 1, game.player_name());

        game.finish_turn();

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

    pub fn find_place_card(&mut self, card: Card) {
        if card != card::EMPTY {
            for i in 0..9 {
                if self.place_card(card, i) {
                    break;
                }
            }
        }
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
