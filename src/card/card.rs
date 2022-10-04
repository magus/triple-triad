use std::fmt;

pub const EMPTY: Card = Card {
    name: "__",
    sides: (0, 0, 0, 0),
    is_player: false,
    is_empty: true,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub name: &'static str,
    pub sides: (u8, u8, u8, u8),
    pub is_player: bool,
    pub is_empty: bool,
}

impl Card {
    pub fn player(name: &'static str, top: u8, right: u8, bottom: u8, left: u8) -> Card {
        Card {
            name,
            sides: (top, right, bottom, left),
            is_player: true,
            is_empty: false,
        }
    }

    pub fn computer(name: &'static str, top: u8, right: u8, bottom: u8, left: u8) -> Card {
        Card {
            name,
            sides: (top, right, bottom, left),
            is_player: false,
            is_empty: false,
        }
    }

    pub fn flip(&mut self, is_player: bool) {
        self.is_player = is_player;
    }

    pub fn top(&self) -> u8 {
        return self.sides.0;
    }

    pub fn right(&self) -> u8 {
        return self.sides.1;
    }

    pub fn bottom(&self) -> u8 {
        return self.sides.2;
    }

    pub fn left(&self) -> u8 {
        return self.sides.3;
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
