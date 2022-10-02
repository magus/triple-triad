use crate::card;
use crate::card::Card;

#[derive(Clone, Debug)]
pub struct Player {
    pub is_player: bool,
    pub cards: [Card; 5],
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

    pub fn find_card(&mut self) -> Card {
        for i in 0..5 {
            if self.cards[i] != card::EMPTY {
                return self.use_card(i);
            }
        }

        return card::EMPTY;
    }

    pub fn use_card(&mut self, index: usize) -> Card {
        let card = self.cards[index];

        if card != card::EMPTY {
            self.cards[index] = card::EMPTY;
        }

        return card;
    }
}
