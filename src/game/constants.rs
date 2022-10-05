pub const ROW_SIZE: usize = 3;
pub const BOARD_SIZE: usize = 9;

pub const CARD_SIDES: usize = 4;

// turn 1: 5 (card choices) to put into 9 (square choices)
// turn 2: 5 (card choices) to put into 8 (square choices)
// turn 3: 4 (card choices) to put into 7 (square choices)
// ...
// turn 9: 1 (card choices) to put into 1 (square choices)
pub const TURN_MOVES: [u8; BOARD_SIZE] = [45, 40, 28, 24, 15, 12, 6, 4, 1];

pub const DEPTH_MOVES: [u64; BOARD_SIZE] = [
    45 * 40 * 28 * 24 * 15 * 12 * 6 * 4 * 1,
    40 * 28 * 24 * 15 * 12 * 6 * 4 * 1,
    28 * 24 * 15 * 12 * 6 * 4 * 1,
    24 * 15 * 12 * 6 * 4 * 1,
    15 * 12 * 6 * 4 * 1,
    12 * 6 * 4 * 1,
    6 * 4 * 1,
    4 * 1,
    1,
];

pub fn max_depth_moves(target_depth: u8, max_depth: i8) -> u64 {
    let is_exhaustive = BOARD_SIZE - (target_depth as usize) <= max_depth as usize;

    // println!(
    //     "target_depth={target_depth}, max_depth={max_depth}, is_exhaustive={is_exhaustive}"
    // );

    if max_depth == 0 || is_exhaustive {
        return total_depth_moves(target_depth);
    }

    let mut total: u64 = 1;

    for i in 0..BOARD_SIZE {
        let depth = target_depth as usize + i;

        // println!("#{i} depth={depth}");

        if i == max_depth as usize - 1 {
            break;
        }

        if depth == BOARD_SIZE {
            break;
        }

        let mult = TURN_MOVES[depth] as u64;
        total *= mult;
        // println!("mult={mult} total={total}");
    }

    return total;
}

pub fn total_depth_moves(depth: u8) -> u64 {
    if depth >= BOARD_SIZE as u8 {
        return 0;
    }

    return DEPTH_MOVES[depth as usize];
}
