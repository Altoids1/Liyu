use std::collections::HashMap;
use std::time::Instant;
use crate::{board::{BoardState, piece::PieceType}, engine::score::INVALID_POS};

use self::score::{ScoreF32, RED_WON,BLACK_WON};

pub mod score;

pub struct Engine
{
    cache : HashMap<BoardState,f32>,
    nodeCount: i32
}

impl Engine {
    fn new() -> Self {
        return Self {
            cache : Default::default(),
            nodeCount : 0
        };
    }
    pub fn evalToDepth(startState : &BoardState, depth : i32) -> ScoreF32 {
        let mut engine : Self = Engine::new();
        let now = Instant::now();
        let ret = engine._eval(startState.to_owned(),depth,&INVALID_POS , &INVALID_POS );
        print!("Engine evaluated {} nodes ({} nodes/sec)\n",engine.nodeCount, (engine.nodeCount as f32) / now.elapsed().as_secs_f32());
        return ret;
    }

    fn inCheck(&self, state : &mut BoardState, kingPos : (usize,usize)) -> bool {
        //TODO
        return false;
    }

    fn _eval(&mut self, state : BoardState, depth : i32, blackBestAbove : &ScoreF32, redBestAbove : &ScoreF32) -> ScoreF32 {
        if depth == 0 {
            return state.getValue();
        }
        /*
        if redBestAbove != INVALID_POS {
            debug_assert!(blackBestAbove <= redBestAbove, "red's best was not higher than black's? {blackBestAbove},{redBestAbove}");
        }
        */
        let moves = state.getAllMoves();
        if moves.is_empty() { // Current player has no moves (and ergo has lost, either by stalemate or checkmate)
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        let kingPos : (usize,usize); // Need this for later when checking for... checks.
        if state.isRedTurn {
            kingPos = state.redPieces.King;
        } else {
            kingPos = state.blackPieces.King;
        }
        let mut foundValidMove : bool = false;
        let mut blackBest : ScoreF32 = ScoreF32::new(f32::INFINITY);
        let mut redBest : ScoreF32 = ScoreF32::new(f32::NEG_INFINITY);

        for (here,there) in moves { // for every possible move
            let mut newBoard = state.branch((here,there)); // apply it to the board
            self.nodeCount += 1;
            if self.inCheck(&mut newBoard, kingPos) { // if we are (or are still) in check in the new position
                continue; // naw
            }
            let moveScore : ScoreF32;
            if foundValidMove {
                moveScore = self._eval(newBoard, depth-1, &blackBest, &redBest);
            } else {
                moveScore = self._eval(newBoard, depth-1, &INVALID_POS, &INVALID_POS);
            }
            foundValidMove = true;
             
            if moveScore == score::INVALID_POS {
                state.branch((here,there)).Display();
                panic!("Position was invalid:");
            }
            if state.isRedTurn { // if current player is red
                if moveScore == score::RED_WON { // and this move just wins
                    return RED_WON; // this is the move
                }
                if redBest < moveScore { // if this move is better than the old best
                    redBest = moveScore; // cool :)
                    if redBest > *blackBestAbove { 
                        //If this results in a position so good that black should've just prevented it from happening
                        //then lets say they did.
                        return redBest;
                    }
                }
            } else {
                if moveScore == score::BLACK_WON {
                    return BLACK_WON;
                }
                if blackBest > moveScore {
                    blackBest = moveScore;
                    if blackBest < *redBestAbove { 
                        //If this results in a position so bad that red should've just prevented it from happening
                        //then lets say they did.
                        return blackBest;
                    }
                }
            }
        }
        if !foundValidMove { // No valid moves means we're checkmated or stalemated, probably
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        
        if state.isRedTurn {
            return redBest;
        } else {
            return blackBest;
        }
    }
}
