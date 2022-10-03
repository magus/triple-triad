use std::fmt;

pub const EMPTY: Card = Card {
    name: "EMPTY",
    sides: (0, 0, 0, 0),
};

#[derive(Copy, Clone, PartialEq)]
pub struct Card {
    pub name: &'static str,
    pub sides: (u8, u8, u8, u8),
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print_name = if self.name == "EMPTY" {
            "__"
        } else {
            self.name
        };
        write!(f, "{}", print_name)
    }
}
