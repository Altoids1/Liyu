#[cfg(test)]

use crate::board;

#[test]
pub fn FEN_starting_position() { // Tests that basic reading/writing of FENs works
    
    let starting_board = board::BoardState::new();
    assert!(starting_board.writeFEN() == board::STARTING_POSITION_FEN);
}

#[test]
pub fn ruleset_test() { // Tests that, at least in the starting position, we output the correct number of moves.
    let starting_board = board::BoardState::new();
    assert_eq!(starting_board.countMoves(),44);
}
