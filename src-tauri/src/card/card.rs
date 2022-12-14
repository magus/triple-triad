use std::fmt;

pub const EMPTY: Card = Card {
    name: "  ",
    id: 0,
    sides: (0, 0, 0, 0),
    modifier: 0,
    tribe: 0,
    is_player: false,
    is_empty: true,
    is_guaranteed: false,
};

#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Card {
    pub name: &'static str,
    pub id: u16,
    pub sides: (u8, u8, u8, u8),
    pub modifier: i8,
    pub tribe: u8,
    pub is_player: bool,
    pub is_empty: bool,
    pub is_guaranteed: bool,
}

impl Card {
    pub const TRIBE_NONE: u8 = 0;
    pub const TRIBE_PRIMAL: u8 = 1;
    pub const TRIBE_SCION: u8 = 2;
    pub const TRIBE_BEASTMAN: u8 = 3;
    pub const TRIBE_GARLEAN: u8 = 4;

    pub fn is_primal(&self) -> bool {
        self.tribe == Card::TRIBE_PRIMAL
    }

    pub fn is_scion(&self) -> bool {
        self.tribe == Card::TRIBE_SCION
    }

    pub fn is_beastman(&self) -> bool {
        self.tribe == Card::TRIBE_BEASTMAN
    }

    pub fn is_garlean(&self) -> bool {
        self.tribe == Card::TRIBE_GARLEAN
    }

    pub fn player_name(size: usize) -> &'static str {
        return match size {
            0 => "P0",
            1 => "P1",
            2 => "P2",
            3 => "P3",
            4 => "P4",
            _ => panic!("unexpected number of player cards [{}]", size),
        };
    }

    pub fn computer_name(size: usize) -> &'static str {
        return match size {
            0 => "C0",
            1 => "C1",
            2 => "C2",
            3 => "C3",
            4 => "C4",
            5 => "C5",
            6 => "C6",
            7 => "C7",
            8 => "C8",
            9 => "C9",
            _ => panic!("unexpected number of npc cards [{}]", size),
        };
    }

    pub fn player(
        name: &'static str,
        id: u16,
        top: u8,
        right: u8,
        bottom: u8,
        left: u8,
        tribe: u8,
    ) -> Card {
        Card {
            name,
            id,
            sides: (top, right, bottom, left),
            modifier: 0,
            tribe,
            is_player: true,
            is_empty: false,
            is_guaranteed: false,
        }
    }

    pub fn computer(
        name: &'static str,
        id: u16,
        top: u8,
        right: u8,
        bottom: u8,
        left: u8,
        tribe: u8,
    ) -> Card {
        Card {
            name,
            id,
            sides: (top, right, bottom, left),
            modifier: 0,
            tribe,
            is_player: false,
            is_empty: false,
            is_guaranteed: false,
        }
    }

    pub fn computer_guaranteed(
        name: &'static str,
        id: u16,
        top: u8,
        right: u8,
        bottom: u8,
        left: u8,
        tribe: u8,
    ) -> Card {
        Card {
            id,
            name,
            sides: (top, right, bottom, left),
            modifier: 0,
            tribe,
            is_player: false,
            is_empty: false,
            is_guaranteed: true,
        }
    }

    pub fn flip(&mut self, is_player: bool) {
        self.is_player = is_player;
    }

    fn value(&self, value: u8) -> u8 {
        if self.modifier == 0 {
            return value;
        }

        return ((value as i8) + self.modifier).min(10).max(1) as u8;
    }

    pub fn top(&self) -> u8 {
        return self.value(self.sides.0);
    }

    pub fn right(&self) -> u8 {
        return self.value(self.sides.1);
    }

    pub fn bottom(&self) -> u8 {
        return self.value(self.sides.2);
    }

    pub fn left(&self) -> u8 {
        return self.value(self.sides.3);
    }

    fn print_side(&self, side: u8) -> String {
        return if self.is_empty {
            " ".to_string()
        } else if side == 10 {
            "A".to_string()
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
