use std::{char::from_digit,fmt};
pub mod piece;
pub mod tile;
use piece::{PieceType,Piece};
use tile::{Tile,TileIterator,PieceIndex};
use crate::engine::score::ScoreF32;
use crate::engine::score;

use self::piece::{PieceSet, PieceSetIterator};

pub type Coord = (usize,usize);
const DEAD_PIECE_COORD : Coord = (usize::MAX,usize::MAX);

/// Is all the information necessary to define a particular state of the board.
#[derive(Clone, PartialEq, Eq)]
pub struct BoardState
{
    // first dimension is x (a to i), second is y (1 to 10)
    pub squares : [[Tile;9];10],
    pub isRedTurn : bool,
    pub plyNumber : i32, // Zero-indexed. Either player moving increments this. Even for Red and odd for Black
    pub(crate) redPieces : PieceSet,
    pub(crate) blackPieces : PieceSet
}


/// the Y index for where black's back rank is.
const BLACK_ROW : usize = 9;
/// the Y index for where red's back rank is.
const RED_ROW : usize = 0;
/// the Y index for where black's river starts.
const BLACK_RIVER : usize = 5;
/// the Y index for where red's river starts.
const RED_RIVER : usize = 4;

pub const STARTING_POSITION_FEN : &str = "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1";

impl BoardState {
    pub fn new() -> Self {
        return Self::new_from_FEN(STARTING_POSITION_FEN);
    }
    pub fn new_from_FEN(fenstr : &str) -> Self { // TODO: Find a good default argument / overloading workaround pattern for Rust
        let mut ret =  Self {
            squares : Default::default(),
            isRedTurn : true,
            plyNumber : 1,
            redPieces : Default::default(),
            blackPieces : Default::default()
        };

        ret.loadFEN(fenstr);

        return ret;
    }

    fn skipWhitespace(iterator : &mut core::str::Chars) -> Option<char>{
        loop {
            let cara = iterator.next();
            if cara.is_none() {
                return cara;
            }
            if !cara.unwrap().is_whitespace() {
                return cara;
            }
        }
    }
    
    fn readNumber(iterator : &mut core::str::Chars) -> i32{
        let mut str : String = Default::default();
        loop {
            let cara = iterator.next();
            if cara.is_none() {
                break;
            }
            if cara.unwrap().is_digit(10) {
                str.push(cara.unwrap());
            }
        }
        let ret = str.parse::<i32>();
        return ret.unwrap_or(1);
    }

    /// Helper of a helper of a helper; for placePiece's eyes only, really.
    fn spawnSpecificPiece<const N: usize>(arr : &mut [(usize,usize);N], coord : &Coord) {
        return BoardState::setSpecificPiece(arr, coord, &DEAD_PIECE_COORD);
    }

    /// Four layers deep of helping, here. Finds the piece in this array that is that the target coordinate and sets its new value.
    fn setSpecificPiece<const N: usize>(arr : &mut [(usize,usize);N], coord : &Coord, targetCoord : &Coord) {
        for (i,oldCoord) in arr.into_iter().enumerate() {
            if *oldCoord == *targetCoord {
                arr[i] = *coord;
                return;
            }
        }
        panic!("Can't find target piece! Agh"); // FIXME: Improve error handling.
    }

    ///To be used exclusively by the FEN reader. Does checking to ensure there aren't too many of any particular piece
    fn spawnPiece(&mut self, cara : char, coord : Coord ) {
        let set : &mut PieceSet;
        let piece : Piece = Piece::new(cara,coord);
        if piece.isRed {
            set = &mut self.redPieces;
        } else {
            set = &mut self.blackPieces;
        }
        self.squares[coord.1][coord.0].pieceIndex = Some(PieceIndex::new(cara));
        match piece.pieceType {
            PieceType::King => {
                set.King = piece.loc;
                self.squares[coord.1][coord.0].pieceIndex = Some(PieceIndex::new(cara));
            },
            PieceType::Rook => Self::spawnSpecificPiece(&mut set.Rooks,&piece.loc),
            PieceType::Cannon => Self::spawnSpecificPiece(&mut set.Cannons,&piece.loc),
            PieceType::Horse => Self::spawnSpecificPiece(&mut set.Horses,&piece.loc),
            PieceType::Elephant => Self::spawnSpecificPiece(&mut set.Elephants,&piece.loc),
            PieceType::Advisor => Self::spawnSpecificPiece(&mut set.Advisors,&piece.loc),
            PieceType::Pawn => Self::spawnSpecificPiece(&mut set.Pawns,&piece.loc),
        };
    }

    pub fn loadFEN(&mut self, fenStr : &str) {
        let mut x : usize = 0;
        let mut y : usize = 9;
        // Doing all this so that we can resume iteration under the metadata for loop later
        let mut iterator = fenStr.chars().into_iter();
        for cara in iterator.by_ref() { // First read in the board
            if cara == '/' {
                y -= 1;
                x = 0;
                continue;
            }
            if cara.is_numeric() {
                x += cara.to_digit(10).unwrap_or(1) as usize;
                continue;
            }
            if cara.is_whitespace() {
                break;
            }
            match cara {
                'p' | 'P' | 'a' | 'A' | 'e' | 'E' | 'h' | 'H' | 'c' | 'C' | 'r' | 'R' | 'k' | 'K'  => {
                    self.spawnPiece(cara,(x,y));
                },
                'N' => {
                    self.spawnPiece('H',(x,y));
                }
                'n' => {
                    self.spawnPiece('h',(x,y));
                }
                ' ' => break,
                _ => {}
            };
            x+=1;
        }
        if y != 0 || x != 9 {
            print!("Warning: FEN was incomplete.");
            return;
        }
        
        let whoseMove = BoardState::skipWhitespace(&mut iterator);
        if whoseMove.is_none() {
            println!("Invalid FEN: missing metadata for whose turn it is");
            return;
        } 
        let cara = whoseMove.unwrap();
        match cara {
            'w' | 'W' | 'r' | 'R' => {
                self.isRedTurn = true;
            }
            'b' | 'B' => {
                self.isRedTurn = false;
            }
            _ => {
                println!("Invalid FEN: move marker not recognized: {}",cara);
            }
        }
        BoardState::skipWhitespace(&mut iterator); // -
        BoardState::skipWhitespace(&mut iterator); // -
        BoardState::readNumber(&mut iterator); // 0
        self.plyNumber = (BoardState::readNumber(&mut iterator) - 1) * 2;
        if !self.isRedTurn { // black's move, so we have 1 extra ply :)
            self.plyNumber += 1;
        }
        if self.redPieces.King == DEAD_PIECE_COORD {
            panic!("Invalid FEN: Red King is missing");
        }
        if self.blackPieces.King == DEAD_PIECE_COORD {
            panic!("Invalid FEN: Black King is missing");
        }

        debug_assert!(self.plyNumber % 2 != (self.isRedTurn as i32)); // ply is even when it's Red's turn and odd when it's Black's

    }

    /// Outputs a FEN which describes the board position.
    pub fn writeFEN(&self) -> String {
        let mut fenString : String = Default::default();
        //"rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1"
        for (index, arr) in self.squares.iter().rev().enumerate() {
            let mut number = 0;
            for tile in arr {
                if tile.pieceIndex.is_none() {
                    number += 1;
                    continue;
                }
                //past here means that there is a piece
                if number != 0 { // first lets write the empty tiles we found earlier :)
                    fenString.push(from_digit(number, 10).unwrap_or('1'));
                    number = 0;
                }
                fenString.push(tile.pieceIndex.as_ref().unwrap().asChar());
            }
            if number != 0 { // push the number
                fenString.push(from_digit(number, 10).unwrap_or('1'));
            }
            if index != 9 {
                fenString.push('/')
            }
        }
        fenString.push(' ');
        if self.isRedTurn {
            fenString.push('w');
        } else {
            fenString.push('b');
        }
        fenString.push_str(" - - 0 ");
        fenString.push_str(((self.plyNumber / 2) + 1).to_string().as_str());

        return fenString;
    }

    pub fn Display(&self) {
        //print!("Position value: {}\n",self.getValue());
        for arr in self.squares.iter().rev() {
            for tile in arr {
                if tile.pieceIndex.is_none() {
                    print!("-");
                    continue;
                }
                print!("{}",tile.pieceIndex.as_ref().unwrap().asChar());
            }
            print!("\n");
        }
    }
    /// Returns the value of the position w/o depth evaluation; the "aesthetic" value of the board.
    /// Positive value means Red is winning, negative value means Black is winning.
    pub fn getValue(&self) -> ScoreF32 {
        let mut sum : f32 = 0f32;
        let mut foundKings = 0;
        let mut foundRed : bool = false;
        let mut foundBlack : bool = false;

        for piece in self.IteratePieces(true) {
            sum += match piece.pieceType {
                PieceType::Pawn => {
                    if piece.loc.1 >= BLACK_RIVER {
                        2f32
                    }
                    else {
                        1f32
                    }
                },
                PieceType::Advisor => 2f32,
                PieceType::Elephant => 2f32,
                PieceType::Horse => 4f32,
                PieceType::Cannon => 4.5f32,
                PieceType::Rook => 9f32,
                PieceType::King => {foundKings+=1;foundRed = true; 0f32} // we handle this differently
            }
        }
        for piece in self.IteratePieces(false) {
            sum -= match piece.pieceType {
                PieceType::Pawn => {
                    if piece.loc.1 <= RED_RIVER {
                        2f32
                    }
                    else {
                        1f32
                    }
                },
                PieceType::Advisor => 2f32,
                PieceType::Elephant => 2f32,
                PieceType::Horse => 4f32,
                PieceType::Cannon => 4.5f32,
                PieceType::Rook => 9f32,
                PieceType::King => {foundKings+=1;foundBlack = true; 0f32} // we handle this differently
            }
        }
        if foundKings > 2 {
            return score::INVALID_POS;
        }
        if !foundBlack {
            return score::RED_WON;
        }
        if !foundRed {
            return score::BLACK_WON;
        }
        return ScoreF32::new(sum);
    }

    fn IsSameColour(&self, x: usize, y : usize, isRed : bool) -> bool {
        if x > 8 || y > 9 {
            panic!("wtf");
        }
        let tile : &Tile = &self.squares[y][x];
        return !tile.pieceIndex.is_none() && tile.pieceIndex.as_ref().unwrap().asChar().is_ascii_uppercase() == isRed;
    }

    fn TryMove(&self, x: usize, y: usize, isRed : bool, moveArr : &mut Vec<Coord> ) {
        if !self.IsSameColour(x, y, isRed) {
            moveArr.push((x,y));
        }
    }
    /// Returns whether the given coordinate is within a palace.
    fn IsPalace(x : usize, y : usize) -> bool {
        return match x {
            3..=5 => {
                match y {
                    0..=2 => true, // Red's palace
                    7..=9 => true, // Black's palace
                    _ => false
                }
            }
            _ => false
        };
    }

    ///NOTE: I'd love for this for be an iterator but iterators MUST be structs in Rust so
    fn GetRaysFrom(&self, x: usize, y : usize) -> [Vec<(Coord, &Tile)>;4] {
        let mut ret : [Vec<(Coord, &Tile)>;4] = Default::default();
        //go up
        for i in y+1..=BLACK_ROW {
            ret[0].push(((x,i),&self.squares[i][x]));
        }
        //go left
        for i in (0..x).rev() { // hate u rust
            ret[1].push(((i,y),&self.squares[y][i]));
        }
        //go right
        for i in x+1..9 {
            ret[2].push(((i,y),&self.squares[y][i]));
        }
        //go down
        for i in (RED_ROW..y).rev() {
            ret[3].push(((x,i),&self.squares[i][x]));
        }

        return ret;
    }

    #[allow(dead_code)] // Just nice to have even if unused atm
    pub fn IterateTiles(&self) -> TileIterator {
        return TileIterator::new(&self.squares);
    }

    pub(crate) fn IteratePieces(&self, isRed : bool) -> PieceSetIterator {
        if isRed {
            return PieceSetIterator::new(&self.redPieces,true);
        }
        return PieceSetIterator::new(&self.blackPieces,false);
    }

    #[allow(dead_code)] // Needed for tests
    pub fn countMoves(&self) -> i32 {
        return self.getAllMoves().len() as i32;
    }

    ///Coordinates returned are in (x,y) order.
    pub fn getAllMoves(&self) -> Vec<(Coord,Coord)> {
        let mut ret : Vec<(Coord,Coord)> = Default::default();
        for piece in self.IteratePieces(self.isRedTurn) {
            for endPos in self.getPieceMoves(&piece) {
                ret.push((piece.loc,endPos));
            }
        }
        return ret;
    }

    ///Creates a new version of the board with the given move played. Implicitly is doing a copy.
    ///Coordinates in (x,y), "from->to" order.
    pub fn branch(&self, newMove : (Coord,Coord)) -> Self {
        let mut ret : Self = self.clone();
        ret.updatePieceLoc(newMove);
        ret.isRedTurn = !ret.isRedTurn;
        ret.plyNumber += 1;
        return ret;
    }

    pub fn hasKing(&self) -> bool {
        if self.isRedTurn {
            return self.redPieces.King != DEAD_PIECE_COORD;
        }
        return self.blackPieces.King != DEAD_PIECE_COORD;
    }

    fn updatePieceLoc(&mut self, newMove : (Coord,Coord)) { // FIXME: Needs to be made faster.
        //Update the tile
        let cara : char;
        if newMove.1 != DEAD_PIECE_COORD { // If we're not moving this piece to heck
            if self.squares[newMove.1.1][newMove.1.0].pieceIndex.as_ref().is_some() { // if a piece is already there
                self.isRedTurn = !self.isRedTurn; // FIXME: wtf
                self.updatePieceLoc((newMove.1,DEAD_PIECE_COORD)); // move it to heck
                self.isRedTurn = !self.isRedTurn; // FIXME: ditto
            }
            let oldTile : &mut Tile = &mut self.squares[newMove.0.1][newMove.0.0];
            self.squares[newMove.1.1][newMove.1.0].pieceIndex = oldTile.pieceIndex.take();
            debug_assert!(self.squares[newMove.1.1][newMove.1.0].pieceIndex.is_some());
            cara = self.squares[newMove.1.1][newMove.1.0].pieceIndex.as_ref().unwrap().cara.to_ascii_lowercase();
        } else {
            cara = self.squares[newMove.0.1][newMove.0.0].pieceIndex.as_ref().unwrap().cara.to_ascii_lowercase();
        }
        //Update the PieceSet location
        let set : &mut PieceSet;
        if self.isRedTurn {
            set = &mut self.redPieces;
        } else {
            set = &mut self.blackPieces;
        };
        match cara {
            'k' => {
                set.King = newMove.1;
            },
            'r' => Self::setSpecificPiece(&mut set.Rooks,&newMove.1,&newMove.0),
            'c' => Self::setSpecificPiece(&mut set.Cannons,&newMove.1,&newMove.0),
            'h' => Self::setSpecificPiece(&mut set.Horses,&newMove.1,&newMove.0),
            'e' => Self::setSpecificPiece(&mut set.Elephants,&newMove.1,&newMove.0),
            'a' => Self::setSpecificPiece(&mut set.Advisors,&newMove.1,&newMove.0),
            'p' => Self::setSpecificPiece(&mut set.Pawns,&newMove.1,&newMove.0),
            _ => unreachable!()
        };
    }

    ///Coordinates returned are in (x,y) order.
    pub fn getPieceMoves(&self, piece : &Piece) -> Vec<Coord> {
        let mut moveArr :  Vec<Coord> = Default::default();
        let x = piece.loc.0;
        let y = piece.loc.1;
        debug_assert_ne!(DEAD_PIECE_COORD,piece.loc);
        match piece.pieceType { 
            PieceType::Pawn => {
                if piece.isRed {
                    //forward
                    if y != BLACK_ROW {
                        self.TryMove(x, y+1, piece.isRed, &mut moveArr);
                    }
                    //sideways
                    if y >= BLACK_RIVER {
                        if x > 0 {
                            self.TryMove(x-1, y, piece.isRed, &mut moveArr);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y, piece.isRed, &mut moveArr);
                        }
                    }

                } else {
                    //forward
                    if y != RED_ROW {
                        self.TryMove(x, y-1, piece.isRed, &mut moveArr);
                    }
                    //sideways
                    if y <= RED_RIVER {
                        if x > 0 {
                            self.TryMove(x-1, y, piece.isRed, &mut moveArr);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y, piece.isRed, &mut moveArr);
                        }
                    }
                }
            }
            PieceType::Advisor => {
                //bounds checking is more lax since advisors can only bump the top & bottom borders, not left & right
                if y != BLACK_ROW {
                    //up & left
                    if BoardState::IsPalace(x-1, y+1) {
                        self.TryMove(x-1, y+1, piece.isRed, &mut moveArr);
                    }
                    //up & right
                    if BoardState::IsPalace(x+1, y+1) {
                        self.TryMove(x+1, y+1, piece.isRed, &mut moveArr);
                    }
                }
                if y != RED_ROW {
                    //down & left
                    if BoardState::IsPalace(x-1, y-1) {
                        self.TryMove(x-1, y-1, piece.isRed, &mut moveArr);
                    }
                    //down & right
                    if BoardState::IsPalace(x+1, y-1) {
                        self.TryMove(x+1, y-1, piece.isRed, &mut moveArr);
                    }
                }
            }
            PieceType::Elephant => {
                //This match is hideous but it's the only way I really know
                //how to communicate this data in a compile-time manner in stock Rust.
                //Note that it does not do a direct isRed check on the elephant;
                //its allegiance is somewhat assumed by which side of the river it's on.
                match (x,y) {
                    //RED ELEPHANTS
                    (2,0) => {
                        self.TryMove(0, 2, piece.isRed, &mut moveArr);
                        self.TryMove(4, 2, piece.isRed, &mut moveArr);
                    }
                    (6,0) => {
                        self.TryMove(8, 2, piece.isRed, &mut moveArr);
                        self.TryMove(4, 2, piece.isRed, &mut moveArr);
                    }
                    (0,2) => {
                        self.TryMove(2, 4, piece.isRed, &mut moveArr);
                        self.TryMove(2, 0, piece.isRed, &mut moveArr);
                    },
                    (4,2) => {
                        self.TryMove(2, 4, piece.isRed, &mut moveArr); //up and left
                        self.TryMove(6, 4, piece.isRed, &mut moveArr); //up and right
                        self.TryMove(2, 0, piece.isRed, &mut moveArr); //down and left
                        self.TryMove(6, 0, piece.isRed, &mut moveArr); //down and right
                    },
                    (8,2) => {
                        self.TryMove(6, 4, piece.isRed, &mut moveArr);
                        self.TryMove(6, 0, piece.isRed, &mut moveArr);
                    },
                    (2,4) => {
                        self.TryMove(0, 2, piece.isRed, &mut moveArr);
                        self.TryMove(4, 2, piece.isRed, &mut moveArr);
                    },
                    (6,4) => {
                        self.TryMove(4, 2, piece.isRed, &mut moveArr);
                        self.TryMove(8, 2, piece.isRed, &mut moveArr);
                    },
                    //BLACK ELEPHANTS
                    (2,9) => {
                        self.TryMove(0, 7, piece.isRed, &mut moveArr);
                        self.TryMove(4, 7, piece.isRed, &mut moveArr);
                    }
                    (6,9) => {
                        self.TryMove(8, 7, piece.isRed, &mut moveArr);
                        self.TryMove(4, 7, piece.isRed, &mut moveArr);
                    }
                    (0,7) => {
                        self.TryMove(2, 5, piece.isRed, &mut moveArr);
                        self.TryMove(2, 9, piece.isRed, &mut moveArr);
                    },
                    (4,7) => {
                        self.TryMove(2, 5, piece.isRed, &mut moveArr); //up and left
                        self.TryMove(6, 5, piece.isRed, &mut moveArr); //up and right
                        self.TryMove(2, 9, piece.isRed, &mut moveArr); //down and left
                        self.TryMove(6, 9, piece.isRed, &mut moveArr); //down and right
                    },
                    (8,7) => {
                        self.TryMove(6, 5, piece.isRed, &mut moveArr);
                        self.TryMove(6, 9, piece.isRed, &mut moveArr);
                    },
                    (2,5) => {
                        self.TryMove(0, 7, piece.isRed, &mut moveArr);
                        self.TryMove(4, 7, piece.isRed, &mut moveArr);
                    },
                    (6,5) => {
                        self.TryMove(4, 7, piece.isRed, &mut moveArr);
                        self.TryMove(8, 7, piece.isRed, &mut moveArr);
                    },
                    _ => {
                        print!("Invalid position for elephant!");
                        debug_assert!(false);
                    }
                };
            }
            PieceType::Horse => {
                //up
                if y < BLACK_ROW - 1 {
                    if self.squares[y+1][x].pieceIndex.is_none() { // Knights can be blocked in Xiangqi!
                        if x > 0 {
                            self.TryMove(x-1, y+2, piece.isRed, &mut moveArr);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y+2, piece.isRed, &mut moveArr);
                        }
                    }
                }
                //left
                if x > 1 {
                    if self.squares[y][x-1].pieceIndex.is_none() {
                        if y < BLACK_ROW {
                            self.TryMove(x-2, y+1, piece.isRed, &mut moveArr);
                        }
                        if y > RED_ROW {
                            self.TryMove(x-2, y-1, piece.isRed, &mut moveArr);
                        }
                    }
                }
                //down
                if y > RED_ROW + 1 {
                    if self.squares[y-1][x].pieceIndex.is_none() {
                        if x > 0 {
                            self.TryMove(x-1, y-2, piece.isRed, &mut moveArr);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y-2, piece.isRed, &mut moveArr);
                        }
                    }
                }
                //right
                if x < 7 {
                    if self.squares[y][x+1].pieceIndex.is_none() {
                        if y < BLACK_ROW {
                            self.TryMove(x+2, y+1, piece.isRed, &mut moveArr);
                        }
                        if y > RED_ROW {
                            self.TryMove(x+2, y-1, piece.isRed, &mut moveArr);
                        }
                    }
                }
            }
            PieceType::Cannon => {
                for ray in self.GetRaysFrom(x,y) {
                    let mut foundHoppable = false;
                    for (pos, tile) in ray {
                        if foundHoppable {
                            if tile.pieceIndex.is_none() {
                                continue;
                            }
                            self.TryMove(pos.0,pos.1, piece.isRed, &mut moveArr);
                            break;
                        } else {
                            if tile.pieceIndex.is_none() {
                                self.TryMove(pos.0, pos.1, piece.isRed, &mut moveArr);
                                continue;
                            }
                            foundHoppable = true;
                        }
                    }
                }
            }
            PieceType::Rook => {
                for ray in self.GetRaysFrom(x,y) {
                    for (pos, tile) in ray {
                        self.TryMove(pos.0, pos.1, piece.isRed, &mut moveArr);   
                        if tile.pieceIndex.is_some() {
                            break;
                        }
                    }
                }
            }
            PieceType::King => {
                //up
                if y < BLACK_ROW && BoardState::IsPalace(x, y+1) {
                    self.TryMove(x, y+1, piece.isRed, &mut moveArr);  
                }
                //down
                if y > RED_ROW && BoardState::IsPalace(x, y-1) {
                    self.TryMove(x, y-1, piece.isRed, &mut moveArr);  
                }
                
                //left
                if x > 0 && BoardState::IsPalace(x-1, y) && !self.shyKing(x-1,y, piece.isRed) {
                    self.TryMove(x-1, y, piece.isRed, &mut moveArr);  
                }
                //right
                if x < 8 && BoardState::IsPalace(x+1, y) && !self.shyKing(x+1,y, piece.isRed){
                    self.TryMove(x+1, y, piece.isRed, &mut moveArr);  
                }
            }
        };

        return moveArr;
    }

    ///Determines whether the shy general rule would prevent this king from moving laterally to the given location.
    fn shyKing(&self, x: usize, y : usize, isRed : bool) -> bool {
        let enemyKingCoords : &Coord;
        if isRed {
            enemyKingCoords = &self.blackPieces.King;
            if x != enemyKingCoords.0 {
                return false;
            }
            for march_y in y..enemyKingCoords.1 {
                if self.squares[march_y][x].pieceIndex.is_some() {
                    return false;
                }
            }
            return true;
        } else {
            enemyKingCoords = &self.redPieces.King;
            if x != enemyKingCoords.0 {
                return false;
            }
            for march_y in enemyKingCoords.1+1..y {
                if self.squares[march_y][x].pieceIndex.is_some() {
                    return false;
                }
            }
            return true;
        }
    }
}

impl std::hash::Hash for BoardState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.squares.hash(state);
        self.isRedTurn.hash(state);
        self.plyNumber.hash(state);
        //Doesn't need to use the PieceSets since they're actually redundant information :)
    }
}
