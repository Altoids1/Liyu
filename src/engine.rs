pub mod score;

use std::cmp::Ordering;
use std::time::Instant;
use crate::board::BoardState;
use crate::board::packedmove::PackedMove;

use self::score::{ScoreF32, RED_WON,BLACK_WON,INVALID_POS};

pub struct Engine
{
    nodeCount: i32,
    recentMoveList : Vec<PackedMove>, // While we do use Vec here, it is definitely preferable to allocate the singular time.
    startStateIsRed : bool,
}

impl Engine {
    fn new() -> Self {
        return Self {
            nodeCount : 0,
            recentMoveList : Default::default(),
            startStateIsRed : Default::default()
        };
    }
    pub fn evalToDepth(startState : &BoardState, depth : i32) -> ScoreF32 {
        let mut engine : Self = Engine::new();
        let now = Instant::now();
        let mut ret : ScoreF32 = ScoreF32::new(0.0f32);
        for i in 1..=depth {
            engine.recentMoveList.push(PackedMove::new());
            if i % 2 != depth % 2 {continue;}
            engine.startStateIsRed = startState.isRedTurn;
            assert_eq!(engine.recentMoveList.len(),i as usize);
            ret = engine._eval_first(startState.to_owned(),i);
            print!("[{}] Engine evaluated {} nodes ({} nodes/sec)\n", i, engine.nodeCount, (engine.nodeCount as f32) / now.elapsed().as_secs_f32());
            //println!("{} move sets recorded",engine.bestCache.len());
            println!("Preferred move: {}",engine.recentMoveList[0]);
        }
        return ret;
    }

    /// Higher value is better.
    /// Super simplistic version of the piece scoring in board.rs.
    fn capture_priority(cara :char) -> i32 {
        match cara {
            'k'|'K' => 999, // Shenzhen I/O moment
            'r'|'R' => 6,
            'c'|'C' => 5,
            'h'|'H' => 4,
            'e'|'E' => 3,
            'a'|'A' => 2,
            'p'|'P' => 1,
            _ => unreachable!("Can't recognize piece given to capture_priority"),
        }
    }

    // Lower value is better.
    fn move_priority(cara : char) -> i32 {
        match cara {
            'k'|'K' => 7,
            'r'|'R' => 4,
            'c'|'C' => 1,
            'h'|'H' => 2,
            'e'|'E' => 5,
            'a'|'A' => 6,
            'p'|'P' => 3,
            _ => unreachable!("Can't recognize piece given to capture_priority"),
        }
    }

    fn check_move_priority(cara : char) -> i32 {
        match cara {
            'k'|'K' => 1,
            'r'|'R' => 3,
            'c'|'C' => 5,
            'h'|'H' => 4,
            'e'|'E' => 6,
            'a'|'A' => 2,
            'p'|'P' => 7,
            _ => unreachable!("Can't recognize piece given to capture_priority"),
        }
    }

    ///Looks at moves A and B and decides which should be evaluated first.
    fn sort_moves(state : &BoardState, inCheck : bool, a : &PackedMove, b : &PackedMove) -> std::cmp::Ordering {
        let alphaPiece = PackedMove::indexStart(&state.squares, a).pieceIndex.asChar();
        let betaPiece = PackedMove::indexStart(&state.squares, b).pieceIndex.asChar();

        // Handle capture preferences
        let betaCapturedPiece = PackedMove::indexEnd(&state.squares, b).pieceIndex.asChar();
        let alphaCapturedPiece = PackedMove::indexEnd(&state.squares, a).pieceIndex.asChar();
        if alphaCapturedPiece != '\0' && betaCapturedPiece != '\0' {
            let alphaCaptureScore = Self::capture_priority(alphaCapturedPiece) - Self::capture_priority(alphaPiece);
            let betaCaptureScore = Self::capture_priority(betaCapturedPiece) - Self::capture_priority(betaPiece);
            let comp = betaCaptureScore.cmp(&alphaCaptureScore);
            if comp != Ordering::Equal {
                return comp;
            }
        }
        // If either is a capture, prefer the capturing move
        else if alphaCapturedPiece != '\0' {
            return Ordering::Less;
        }
        else if betaCapturedPiece != '\0' {
            return Ordering::Greater;
        }
        if inCheck {
            return Self::check_move_priority(alphaPiece).cmp(&Self::check_move_priority(betaPiece));
        }
        return Self::move_priority(alphaPiece).cmp(&Self::move_priority(betaPiece));
    }

    fn recordRecentMove(&mut self, packedMove : PackedMove, depth : i32) {
        if packedMove == PackedMove::new() {
            return;
        }

        let len = self.recentMoveList.len() as i32;
        self.recentMoveList[(len - depth) as usize] = packedMove;
    }

    fn _eval_first(&mut self, state : BoardState, depth : i32) -> ScoreF32 {
        return self._eval(state,depth, &INVALID_POS, &INVALID_POS);
    }

    fn _eval(&mut self, state : BoardState, depth : i32, blackBestAbove : &ScoreF32, redBestAbove : &ScoreF32) -> ScoreF32 {
        if depth == 0 {
            let val = state.getValue();
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

        let inCheck = state.isInCheck();

        moves.sort_unstable_by(|a,b| { // Awkward to wrap this function call in a closure but whaaatever
            if *a == self.recentMoveList[0] {
                return Ordering::Less;
            }
            if *b == self.recentMoveList[0] {
                return Ordering::Greater;
            }
            Self::sort_moves(&state, inCheck, a, b)
        });

        let mut foundValidMove : bool = false;
        let mut ourBest : ScoreF32;
        let mut ourBestMove : PackedMove = PackedMove::new();
        if state.isRedTurn {
            ourBest = BLACK_WON;
        } else {
            ourBest = RED_WON;
        }

        for packedMove in moves { // for every possible move
            //debug_assert!(here.0 < 9);
            //debug_assert!(here.1 < 10);
            let newBoard = state.branch(packedMove); // apply it to the board
            self.nodeCount += 1;
            let moveScore : ScoreF32;
            if !newBoard.hasKing() {
                if state.isRedTurn {
                    moveScore = RED_WON;
                } else {
                    moveScore = BLACK_WON;
                }
            }
            else {
                if state.isRedTurn {
                    moveScore = self._eval(newBoard, depth-1, &blackBestAbove, ourBest.shitty_max(redBestAbove));
                } else {
                    moveScore = self._eval(newBoard, depth-1, ourBest.shitty_min(blackBestAbove), &redBestAbove);
                }
                foundValidMove = true;
            }
            if moveScore == score::INVALID_POS {
                //state.branch((here,there)).Display();
                continue;
                //panic!("Position was invalid:");
            }
            
            if state.isRedTurn { // if current player is red
                if moveScore > ourBest { // if this move is better than the old best
                    if moveScore > *blackBestAbove { 
                        //If this results in a position so good that black should've just prevented it from happening
                        //then lets say they did.
                        //println!("*snip for Red ({} vs {})",moveScore,blackBestAbove);
                        return *blackBestAbove;
                    }
                    ourBest = moveScore; // cool :)
                    ourBestMove = packedMove;
                }
                if moveScore == score::RED_WON { // if this move just wins then we're done
                    break;
                }
                
            } else { // Current player is black
                if moveScore < ourBest {
                    if moveScore < *redBestAbove { 
                        //If this results in a position so bad that red should've just prevented it from happening
                        //then lets say they did.
                        //println!("*snip for Black ({} vs {})",moveScore,redBestAbove);
                        return *redBestAbove;
                    }
                    ourBest = moveScore;
                    ourBestMove = packedMove;
                }
                if moveScore == score::BLACK_WON {
                    break;
                }
            }
        }
        self.recordRecentMove(ourBestMove, depth);
        if !foundValidMove { // No valid moves means we're checkmated or stalemated, probably
            if state.isRedTurn {
                return score::BLACK_WON;
            }
            return score::RED_WON;
        }
        return ourBest;
    }
}
