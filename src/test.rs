
#![cfg(test)]

use crate::board;
use crate::engine;
use crate::engine::packedmove::PackedMove;
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
pub fn ruleset_shy_general() { // Tests that the shy general rule works
    let board = board::BoardState::new_from_FEN("3k5/9/9/4p4/9/9/4P4/9/9/4K4 w - - 0 1");
    assert_eq!(board.countMoves(),3);
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
pub fn iterator_asserts() { // Tests that iterating tiles & pieces both work :)
    let starting_board = board::BoardState::new();
    assert_eq!(starting_board.IterateTiles().count(),90);
    assert_eq!(starting_board.IteratePieces(starting_board.isRedTurn).count(),16);
}

#[test]
pub fn packedmove_test() {
    let packer : PackedMove = PackedMove::new_from_Coords(((0,0),(0,1)));
    assert_eq!(format!("{}",packer),"a1a2");
}

#[test]
pub fn engine_starting_position() {
    let starting_board = board::BoardState::new();
    let score = engine::Engine::evalToDepth(&starting_board,3);
    let displayResult = format!("{}",score);
    //we can't make too many assertions about what the engine thinks about the starting position,
    //but there are a few obvious things we can do here
    assert_ne!(score,INVALID_POS);
    assert_ne!(displayResult,"inf");
    assert_ne!(displayResult,"-inf");
}

#[test]
pub fn engine_mated_position() {
    let mate_one = board::BoardState::new_from_FEN("R3k4/R8/9/9/9/9/9/9/9/5K3 b - - 0 22"); // backrank mate
    let mate_one_score = engine::Engine::evalToDepth(&mate_one, 4);
    assert_eq!(mate_one_score,RED_WON);
}

#[test]
pub fn engine_mate_in_one() {
    let mate_one = board::BoardState::new_from_FEN("2eakaer1/4h4/4H1h2/p1P1p1p1p/9/8P/P5P2/E3C1H1C/6r2/3AKAE1R r - - 0 22"); // smothered mate
    let mate_one_score = engine::Engine::evalToDepth(&mate_one, 4);
    assert_eq!(mate_one_score,RED_WON);
}

#[test]
#[ignore] // Too slow right now!
pub fn engine_mate_in_two() {
    let mate_two = board::BoardState::new_from_FEN("4P4/4ak3/1r4N2/6p1p/4c4/6P2/Pc3r2P/4CR3/4A4/1RBK1ABN1 w - - 0 1"); // Mate in two (with pins)
    let mate_two_score = engine::Engine::evalToDepth(&mate_two, 6);
    assert_eq!(mate_two_score,RED_WON);
}