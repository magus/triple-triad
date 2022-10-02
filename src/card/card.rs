pub const EMPTY: Card = Card {
    name: "EMPTY",
    sides: (0, 0, 0, 0),
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub name: &'static str,
    pub sides: (u8, u8, u8, u8),
}
