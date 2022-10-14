#[cfg(test)]

use crate::board;

#[test]
pub fn FEN_starting_position() { // Tests that basic reading/writing of FENs works
    
    let starting_board = board::BoardState::new();
    assert!(starting_board.writeFEN() == "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1");
}
