
#![cfg(test)]

use crate::board;
use crate::engine;
use crate::engine::score::{ScoreF32,BLACK_WON,INVALID_POS,RED_WON};

#[test]
pub fn FEN_starting_position() { // Tests that basic reading/writing of FENs works
    
    let starting_board = board::BoardState::new();
    assert_eq!(starting_board.writeFEN(),board::STARTING_POSITION_FEN);
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


#[test]
pub fn score_test() {
    assert_eq!(format!("{:#b}",engine::score::RED_WON),"0b1111111111000000000000000000000");
    assert_eq!(INVALID_POS,INVALID_POS);
    assert!(!(INVALID_POS != INVALID_POS));
    assert_ne!(INVALID_POS,RED_WON);
    assert_ne!(BLACK_WON,RED_WON);
    assert!(BLACK_WON < RED_WON);
    assert!(BLACK_WON < ScoreF32::new(0f32));
    assert!(RED_WON > ScoreF32::new(5f32));
}

#[test]
pub fn tileiterator_asserts() {
    let starting_board = board::BoardState::new();
    assert_eq!(starting_board.IterateTiles().count(),90);
}

#[test]
pub fn engine_starting_position() {
    let starting_board = board::BoardState::new();
    let displayResult = format!("{}",engine::Engine::evalToDepth(&starting_board,3));
    assert_ne!(displayResult,"inf");
    assert_ne!(displayResult,"-inf");
}