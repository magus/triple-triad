use crate::card;
use crate::card::Card;

const PLAYER_HAND_SIZE: usize = 5;

#[derive(Clone, Debug)]
pub struct Player {
    pub cards: [Card; PLAYER_HAND_SIZE],
}

impl Player {
    pub fn cards_left(&self, maybe_chaos_card: Option<usize>) -> Vec<usize> {
        let mut card_index_list = vec![];

        // handle potential chaos flagged card
        if let Some(chaos_card) = maybe_chaos_card {
            if self.cards[chaos_card] != card::EMPTY {
                // valid chaos card, return it
                return vec![chaos_card];
            }
        }

        // otherwise, return all cards in hand
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

const MAX_COMPUTER_HAND_SIZE: usize = 8;

#[derive(Clone, Debug)]
pub struct Computer {
    pub cards: [Card; MAX_COMPUTER_HAND_SIZE],
    pub cards_used: usize,
}

impl Computer {
    pub fn cards_left(&self) -> Vec<usize> {
        let mut card_index_list = vec![];

        for i in 0..MAX_COMPUTER_HAND_SIZE {
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
            self.cards_used += 1;
        }

        return card;
    }

    // computer can have between 6-8 cards it seems
    // of those between 2-4 seem to be guaranteed
    // we can use this fact to predict remaining cards
    //
    // examples
    // https://arrtripletriad.com/en/npc-droyn
    // https://arrtripletriad.com/en/npc-aiglephine
    // https://arrtripletriad.com/en/npc-prudence
    // https://arrtripletriad.com/en/npc-ghasa
    pub fn maybe_filter_hand_guaranteed(&mut self) {
        // count how many cards in hand are left
        let hand_remaining = PLAYER_HAND_SIZE - self.cards_used;

        // count how many guaranteed cards are remaining
        let mut guaranteed_remaining = vec![];

        for i in 0..MAX_COMPUTER_HAND_SIZE {
            if self.cards[i].is_guaranteed {
                guaranteed_remaining.push(i);
            }
        }

        // if equal, remove non-guaranteed cards from hand
        if hand_remaining == guaranteed_remaining.len() {
            // println!("🚨 Guaranteed remaining cards!");

            for i in 0..MAX_COMPUTER_HAND_SIZE {
                if !self.cards[i].is_guaranteed {
                    self.cards[i] = card::EMPTY;
                }
            }
        }
    }
}
