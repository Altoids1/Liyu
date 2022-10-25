pub mod score;
pub mod packedmove;

use std::{collections::HashMap, cmp::Ordering};
use std::time::Instant;
use crate::{board::{BoardState, Coord}, engine::score::INVALID_POS};

use self::score::{ScoreF32, RED_WON,BLACK_WON};

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

    ///Lower is better.
    fn capture_priority(cara :char) -> i32 {
        match cara {
            'k'|'K' => 0,
            'r'|'R' => 1,
            'c'|'C' => 2,
            'h'|'H' => 3,
            'e'|'E' => 4,
            'a'|'A' => 5,
            'p'|'P' => 6,
            _ => 7,
        }
    }

    ///Looks at moves A and B and decides which should be evaluated first.
    fn sort_moves(state : &BoardState, a : &(Coord,Coord), b : &(Coord,Coord)) -> std::cmp::Ordering {
        //Does B capture anything?
        if state.squares[b.1.1][b.1.0].pieceIndex.is_some() {
            //If A doesn't capture yet B does capture
            if state.squares[a.1.1][a.1.0].pieceIndex.is_none() {
                return Ordering::Greater; // then B should be first
            }
            //if both capture, we have a priority system
            let caraAlpha : char = state.squares[a.1.1][a.1.0].pieceIndex.as_ref().unwrap().asChar();
            let caraBeta : char = state.squares[b.1.1][b.1.0].pieceIndex.as_ref().unwrap().asChar();
            return Self::capture_priority(caraAlpha).cmp(&Self::capture_priority(caraBeta));
        }
        //If B doesn't capture anything, then whatever A is, it should go first.
        return Ordering::Less; // Preserve old order, I guess!
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
        let mut moves = state.getAllMoves();
        moves.sort_unstable_by(|a,b| { // Awkward to wrap this function call in a closure but whaaatever
            Self::sort_moves(&state, a, b)
        });
        if moves.is_empty() { // Current player has no moves (and ergo has lost, either by stalemate or checkmate)
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        let mut foundValidMove : bool = false;
        let mut blackBest : ScoreF32 = RED_WON;
        let mut redBest : ScoreF32 = BLACK_WON;

        for (here,there) in moves { // for every possible move
            debug_assert!(here.0 < 9);
            debug_assert!(here.1 < 10);
            let newBoard = state.branch((here,there)); // apply it to the board
            self.nodeCount += 1;
            if !newBoard.hasKing() {
                if state.isRedTurn {
                    return RED_WON;
                } else {
                    return BLACK_WON;
                }
            }
            let moveScore : ScoreF32;
            if foundValidMove {
                moveScore = self._eval(newBoard, depth-1, &blackBest, &redBest);
            } else {
                moveScore = self._eval(newBoard, depth-1, &INVALID_POS, &INVALID_POS);
            }
            if moveScore == score::INVALID_POS {
                //state.branch((here,there)).Display();
                continue;
                //panic!("Position was invalid:");
            }
            foundValidMove = true;
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
