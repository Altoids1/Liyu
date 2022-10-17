use std::{collections::HashMap};
use crate::board::{BoardState, piece::PieceType};

use self::score::{ScoreF32, RED_WON,BLACK_WON};

pub mod score;

pub struct Engine
{
    cache : HashMap<BoardState,f32>
}

impl Engine {
    fn new() -> Self {
        return Self {
            cache : Default::default()
        };
    }
    pub fn evalToDepth(startState : &BoardState, depth : i32) -> ScoreF32 {
        let mut engine : Self = Engine::new();
        return engine._eval(startState.to_owned(),depth);
    }

    fn findKing(&self, state : &BoardState) -> Result<(usize, usize), i32> {
        for (coords, tile) in state.IterateTiles() {
            if tile.piece.is_some() {
                let piece = tile.piece.as_ref().unwrap();
                if piece.isRed == state.isRedTurn && piece.pieceType == PieceType::King { 
                    return Ok(coords);
                }
            }
        }
        return Err(-1);
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
        let kingRet = self.findKing(&state);
        if kingRet.is_err() {
            return score::INVALID_POS;
        }
        let kingPos = kingRet.unwrap();
        let mut foundValidMove : bool = false;
        let mut blackBest : ScoreF32 = ScoreF32::new(f32::INFINITY);
        let mut redBest : ScoreF32 = ScoreF32::new(f32::NEG_INFINITY);

        for (here,there) in moves {
            let newBoard = state.branch((here,there));
            if self.inCheck(&mut state, kingPos) {
                continue;
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
