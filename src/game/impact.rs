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
