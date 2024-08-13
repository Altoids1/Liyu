pub mod score;

use std::{collections::HashMap, cmp::Ordering};
use std::time::Instant;
use crate::board::BoardState;
use crate::board::packedmove::PackedMove;

use self::score::{ScoreF32, RED_WON,BLACK_WON,INVALID_POS};

pub struct Engine
{
    ///This is used to help Liyu remember what moves she took. <br/>
    ///Key is the score of this permutation, value is the moves it took to get there
    bestCache : HashMap<ScoreF32,Vec<PackedMove>>,
    nodeCount: i32,
    recentMoveList : Vec<PackedMove>, // While we do use Vec here, it is definitely preferable to allocate the singular time.
    startStateIsRed : bool,
}

impl Engine {
    fn new() -> Self {
        return Self {
            bestCache : Default::default(),
            nodeCount : 0,
            recentMoveList : Default::default(),
            startStateIsRed : Default::default()
        };
    }
    pub fn evalToDepth(startState : &BoardState, depth : i32) -> ScoreF32 {
        let mut engine : Self = Engine::new();
        let now = Instant::now();
        engine.recentMoveList = vec![PackedMove::new(); depth as usize];
        engine.startStateIsRed = startState.isRedTurn;
        assert_eq!(engine.recentMoveList.len(),depth as usize);
        let ret = engine._eval_first(startState.to_owned(),depth);
        print!("Engine evaluated {} nodes ({} nodes/sec)\n",engine.nodeCount, (engine.nodeCount as f32) / now.elapsed().as_secs_f32());
        if engine.bestCache.is_empty() {
            println!("No moves were available!");
        } else {
            //println!("{} move sets recorded",engine.bestCache.len());
            print!("Moves: ");
            for goodMove in engine.bestCache[&ret].iter() {
                print!(" {} ",goodMove);
            }
            print!("\n");
        }
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
    fn sort_moves(state : &BoardState, a : &PackedMove, b : &PackedMove) -> std::cmp::Ordering {
        //Does B capture anything?
        let betaCaptures : char = PackedMove::indexEnd(&state.squares, b).pieceIndex.asChar();
        if betaCaptures != '\0' {
            let alphaCaptures : char = PackedMove::indexEnd(&state.squares, a).pieceIndex.asChar();
            //If A doesn't capture yet B does capture
            if alphaCaptures == '\0' {
                return Ordering::Greater; // then B should be first
            }
            //if both capture, we have a priority system
            let captureCmp = Self::capture_priority(alphaCaptures).cmp(&Self::capture_priority(betaCaptures));
            if captureCmp == Ordering::Equal { // If they're capturing the same tier of piece
                let alphaPiece = PackedMove::indexStart(&state.squares, a).pieceIndex.asChar();
                let betaPiece =  PackedMove::indexStart(&state.squares, b).pieceIndex.asChar();
                // The comparison order here is reversed to prefer the attacker of lowest value.
                return Self::capture_priority(betaPiece).cmp(&Self::capture_priority(alphaPiece)); 
            }
            return captureCmp;
        }
        //If B doesn't capture anything, then whatever A is, it should go first.
        return Ordering::Less; // Preserve old order, I guess!
    }

    fn recordRecentMove(&mut self, packedMove : PackedMove, depth : i32) {
        if packedMove == PackedMove::new() {
            return;
        }

        let len = self.recentMoveList.len() as i32;
        self.recentMoveList[(len - depth) as usize] = packedMove;
    }

    fn cacheMoveOrder(&mut self, score : ScoreF32) {
        if !self.bestCache.contains_key(&score) { // Bad to double-hash but it does avoid a clone that WOULD happen with or_insert()
            self.bestCache.insert(score, self.recentMoveList.clone());
        }
    }

    fn _eval_first(&mut self, state : BoardState, depth : i32) -> ScoreF32 {
        return self._eval(state,depth, &INVALID_POS, &INVALID_POS);
    }

    fn _eval(&mut self, state : BoardState, depth : i32, blackBestAbove : &ScoreF32, redBestAbove : &ScoreF32) -> ScoreF32 {
        if depth == 0 {
            let val = state.getValue();
            self.cacheMoveOrder(val);
            return val;
        }

        if redBestAbove != INVALID_POS && blackBestAbove != INVALID_POS {
            
            if redBestAbove == blackBestAbove { // alpha-beta collapse!
                return *redBestAbove;
            }
            //println!("{} and {}", redBestAbove, blackBestAbove);
            //debug_assert!(depth != 6 && blackBestAbove <= redBestAbove, "red's best was not higher than black's? {blackBestAbove},{redBestAbove}");
        }

        let mut moves = state.getAllMoves();
        if moves.is_empty() { // Current player has no moves (and ergo has lost, either by stalemate or checkmate)
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        moves.sort_unstable_by(|a,b| { // Awkward to wrap this function call in a closure but whaaatever
            Self::sort_moves(&state, a, b)
        });

        let mut foundValidMove : bool = false;
        let mut blackBest : ScoreF32 = RED_WON;
        let mut redBest : ScoreF32 = BLACK_WON;

        for packedMove in moves { // for every possible move
            //debug_assert!(here.0 < 9);
            //debug_assert!(here.1 < 10);
            let newBoard = state.branch(packedMove); // apply it to the board
            self.nodeCount += 1;
            self.recordRecentMove(packedMove, depth);
            let moveScore : ScoreF32;
            if !newBoard.hasKing() {
                if state.isRedTurn {
                    moveScore = RED_WON;
                } else {
                    moveScore = BLACK_WON;
                }
                self.cacheMoveOrder(moveScore);
            }
            else if foundValidMove {
                moveScore = self._eval(newBoard, depth-1, &blackBest, &redBest);
            } else {
                moveScore = self._eval(newBoard, depth-1, &INVALID_POS, &INVALID_POS);
                foundValidMove = true;
            }
            if moveScore == score::INVALID_POS {
                //state.branch((here,there)).Display();
                continue;
                //panic!("Position was invalid:");
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
                self.cacheMoveOrder(BLACK_WON);
                return score::BLACK_WON;
            }
            self.cacheMoveOrder(BLACK_WON);
            return score::RED_WON;
        }
        
        if state.isRedTurn {
            return redBest;
        } else {
            return blackBest;
        }
    }
}
