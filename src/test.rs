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

#[test]
pub fn ruleset_branch() { // Tests that, like, moving pieces around works
    let mut starting_board = board::BoardState::new();
    starting_board = starting_board.branch(((0,3),(0,4))); // Move a pawn!
    assert_eq!(starting_board.writeFEN(),"rheakaehr/9/1c5c1/p1p1p1p1p/9/P8/2P1P1P1P/1C5C1/9/RHEAKAEHR b - - 0 1");
}