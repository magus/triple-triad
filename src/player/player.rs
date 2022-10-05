use crate::card;
use crate::card::Card;

type Hand = [Card; HAND_SIZE];

#[derive(Clone, Debug)]
pub struct Player {
    pub is_player: bool,
    pub cards: Hand,
}

impl Player {
    pub fn cards_left(&self) -> Vec<usize> {
        let mut card_index_list = vec![];

        for i in 0..5 {
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

const HAND_SIZE: usize = 5;
