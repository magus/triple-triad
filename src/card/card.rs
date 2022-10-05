use std::fmt;

pub const EMPTY: Card = Card {
    name: "  ",
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

    fn print_side(&self, side: u8) -> String {
        return if self.is_empty {
            " ".to_string()
        } else {
            format!("{}", side)
        };
    }

    pub fn print_top(&self) -> String {
        self.print_side(self.sides.0)
    }

    pub fn print_right(&self) -> String {
        self.print_side(self.sides.1)
    }

    pub fn print_bottom(&self) -> String {
        self.print_side(self.sides.2)
    }

    pub fn print_left(&self) -> String {
        self.print_side(self.sides.3)
    }

    pub fn rgb_color(&self) -> ((u8, u8, u8), (u8, u8, u8)) {
        return if self.is_empty {
            ((0, 0, 0), (140, 140, 140))
        } else if self.is_player {
            ((200, 200, 200), (65, 88, 120))
        } else {
            ((200, 200, 200), (95, 40, 50))
        };
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (top, right, bottom, left) = self.sides;
        write!(f, "{}[{},{},{},{}]", self.name, top, right, bottom, left)
    }
}
