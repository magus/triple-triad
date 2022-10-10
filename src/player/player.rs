use crate::card;
use crate::card::Card;

const PLAYER_HAND_SIZE: usize = 5;

#[derive(Clone, Debug)]
pub struct Player {
    pub cards: [Card; PLAYER_HAND_SIZE],
}

impl Player {
    pub fn cards_left(&self) -> Vec<usize> {
        let mut card_index_list = vec![];

        for i in 0..PLAYER_HAND_SIZE {
            if self.cards[i] != card::EMPTY {
                card_index_list.push(i);
            }
        }

        return card_index_list;
    }

    pub fn use_card(&mut self, index: usize) -> Card {
        let card = self.cards[index];

        if card != card::EMPTY {
            self.cards[index] = card::EMPTY;
        }

        return card;
    }
}

const COMPUTER_HAND_SIZE: usize = 7;

#[derive(Clone, Debug)]
pub struct Computer {
    pub cards: [Card; COMPUTER_HAND_SIZE],
}

impl Computer {
    pub fn cards_left(&self) -> Vec<usize> {
        let mut card_index_list = vec![];

        for i in 0..COMPUTER_HAND_SIZE {
            if self.cards[i] != card::EMPTY {
                card_index_list.push(i);
            }
        }

        return card_index_list;
    }

    pub fn use_card(&mut self, index: usize) -> Card {
        let card = self.cards[index];

        if card != card::EMPTY {
            self.cards[index] = card::EMPTY;
        }

        return card;
    }
}
