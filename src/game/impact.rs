use crate::game::constants::BOARD_SIZE;
use crate::game::constants::CARD_SIDES;

// map to surrounding squares
// [top, right, bottom, left]
pub static BOARD: [[Option<usize>; CARD_SIDES]; BOARD_SIZE] = [
    [None, Some(1), Some(3), None],
    [None, Some(2), Some(4), Some(0)],
    [None, None, Some(5), Some(1)],
    [Some(0), Some(4), Some(6), None],
    [Some(1), Some(5), Some(7), Some(3)],
    [Some(2), None, Some(8), Some(4)],
    [Some(3), Some(7), None, None],
    [Some(4), Some(8), None, Some(6)],
    [Some(5), None, None, Some(7)],
];

#[derive(Debug, Copy, Clone)]
pub struct ImpactPair {
    // the target of the impact, i.e. square that may be flipped
    pub square: usize,
    // the source card value for this impact side
    pub card: u8,
    // the target card value for this impact side
    pub impacted: u8,
    // used for plus rule
    pub sum: u8,
}

impl ImpactPair {
    pub fn new(square: usize, card: u8, impacted: u8) -> ImpactPair {
        let sum = card + impacted;

        ImpactPair {
            square,
            card,
            impacted,
            sum,
        }
    }
}
