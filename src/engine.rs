use std::collections::HashMap;
use std::time::Instant;
use crate::board::{BoardState, piece::PieceType};

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
        let ret = engine._eval(startState.to_owned(),depth);
        print!("Engine evaluated {} nodes ({} nodes/sec)\n",engine.nodeCount, (engine.nodeCount as f32) / now.elapsed().as_secs_f32());
        return ret;
    }

    fn inCheck(&self, state : &mut BoardState, kingPos : (usize,usize)) -> bool {
        //TODO
        return false;
    }

    fn _eval(&mut self, mut state : BoardState, depth : i32) -> ScoreF32 {
        if depth == 0 {
            return ScoreF32::new(state.getValue());
        }

        let moves = state.getAllMoves();
        if moves.is_empty() { // Current player has no moves (and ergo has lost)
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        //Is the current player's king attacked?
        let kingPos : (usize,usize);
        if state.isRedTurn {
            kingPos = state.redPieces.King;
        } else {
            kingPos = state.blackPieces.King;
        }
        let mut foundValidMove : bool = false;
        let mut blackBest : ScoreF32 = ScoreF32::new(f32::INFINITY);
        let mut redBest : ScoreF32 = ScoreF32::new(f32::NEG_INFINITY);

        for (here,there) in moves {
            let mut newBoard = state.branch((here,there));
            self.nodeCount += 1;
            if self.inCheck(&mut newBoard, kingPos) { // if we are (or are still) in check in the new position
                continue; // naw
            }
            foundValidMove = true;
            let moveScore = self._eval(newBoard, depth-1);
            if moveScore == score::INVALID_POS {
                continue; // wtf?
            }
            if state.isRedTurn {
                if moveScore == score::RED_WON {
                    return RED_WON;
                }
                if redBest.data < moveScore.data {
                    redBest = moveScore;
                }
            } else {
                if moveScore == score::BLACK_WON {
                    return BLACK_WON;
                }
                if blackBest.data > moveScore.data {
                    blackBest = moveScore;
                }
            }
        }
        if !foundValidMove { // No valid moves means we're checkmated, probably
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
