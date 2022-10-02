use crate::card;
use crate::card::Card;

#[derive(Clone, Debug)]
pub struct Player {
    pub is_player: bool,
    pub cards: [Card; 5],
}

impl Player {
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
            println!("setting {} to empty", index);
            // set this card in player cards to empty (used)
            self.cards[index] = card::EMPTY;
            println!("cards {:?}", self.cards);
        }

        return card;
    }
}
