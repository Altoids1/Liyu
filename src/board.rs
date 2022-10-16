use std::{char::from_digit,fmt};
pub mod piece;
pub mod tile;
use piece::{PieceType,Piece};
use tile::{Tile,TileIterator};

/// Is all the information necessary to define a particular state of the board.
pub struct BoardState
{
    // first dimension is x (a to i), second is y (1 to 10)
    squares : [[Tile;9];10],
    isRedTurn : bool,
    plyNumber : i32, // Zero-indexed. Either player moving increments this. Even for Red and odd for Black
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
            plyNumber : 1
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
                    _ = self.squares[y][x].piece.insert(Piece::new(cara));
                },
                ' ' => break,
                _ => {}
            }
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

        debug_assert!(self.plyNumber % 2 != (self.isRedTurn as i32)); // ply is even when it's Red's turn and odd when it's Black's

    }

    /// Outputs a FEN which describes the board position.
    pub fn writeFEN(&self) -> String {
        let mut fenString : String = Default::default();
        //"rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1"
        for (index, arr) in self.squares.iter().rev().enumerate() {
            let mut number = 0;
            for tile in arr {
                if tile.piece.is_none() {
                    number += 1;
                    continue;
                }
                //past here means that there is a piece
                if number != 0 { // first lets write the empty tiles we found earlier :)
                    fenString.push(from_digit(number, 10).unwrap_or('1'));
                    number = 0;
                }
                let piece : &Piece = tile.piece.as_ref().unwrap();
                fenString.push(piece.getChar());
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
        print!("Position value: {}\n",self.getValue());
        for arr in self.squares.iter().rev() {
            for tile in arr {
                if tile.piece.is_none() {
                    print!("-");
                    continue;
                }
                print!("{}",tile.piece.as_ref().unwrap().getChar());
            }
            print!("\n");
        }
    }
    /// Returns the value of the position w/o depth evaluation; the "aesthetic" value of the board.
    /// Positive value means Red is winning, negative value means Black is winning. Forced draws are 0.
    pub fn getValue(&self) -> f32 {
        let mut sum : f32 = 0f32;
        let mut foundRedKing : bool = false;
        let mut foundBlackKing : bool = false;
        for arr in self.squares.iter() { // TODO: Make this more complicated than just piece value summing
            for tile in arr {
                if tile.piece.is_none() {
                    continue;
                }
                let piece : &Piece = &tile.piece.as_ref().unwrap();
                let mut ourVal: f32 = match piece.pieceType {
                    PieceType::Pawn => 1f32, // TODO: Increase value after they cross the river
                    PieceType::Advisor => 2f32,
                    PieceType::Elephant => 2f32,
                    PieceType::Horse => 4f32,
                    PieceType::Cannon => 4.5f32,
                    PieceType::Rook => 9f32,
                    PieceType::King => { // We treat this differently :3
                        if piece.isRed {
                            foundRedKing = true;
                            0f32
                        } else {
                            foundBlackKing = true;
                            0f32
                        }
                    }
                };
                if !piece.isRed {
                    ourVal *= -1f32;
                }
                sum += ourVal;
            }
        }
        if !foundBlackKing { // red wins innit
            if !foundRedKing { // wtf
                debug_assert!(false);
            }
            sum = f32::INFINITY;
        }
        if !foundRedKing { // black wins innit
            sum = f32::NEG_INFINITY;
        }
        return sum;
    }

    fn IsSameColour(&self, x: usize, y : usize, isRed : bool) -> bool {
        return !self.squares[y][x].piece.is_none() && self.squares[y][x].piece.as_ref().unwrap().isRed == isRed;
    }

    fn TryMove(&self, x: usize, y: usize, isRed : bool, flagBoard : &mut [[bool;9];10] ) {
        if !self.IsSameColour(x, y, isRed) {
            flagBoard[y][x] = true;
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
    fn GetRaysFrom(&self, x: usize, y : usize) -> [Vec<((usize, usize), &Tile)>;4] {
        let mut ret : [Vec<((usize, usize), &Tile)>;4] = Default::default();
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

    pub fn IterateTiles(&self) -> TileIterator {
        return TileIterator::new(&self.squares);
    }

    pub fn countMoves(&self) -> i32 {
        let mut count = 0;
        for (y,arr) in self.squares.iter().enumerate() {
            for (x,tile) in arr.iter().enumerate() {
                if tile.piece.is_none() {
                    continue;
                }
                let piece : &Piece = tile.piece.as_ref().unwrap();
                if piece.isRed != self.isRedTurn {
                    continue;
                }
                let arr = self.getMoves(x, y);
                let mut localCount = 0;
                for sub in arr.iter() {
                    for val in sub.iter() {
                        if *val {
                            localCount += 1;
                        }
                    }
                }
                println!("{} has {} moves!",piece,localCount);
                count += localCount;
            }
        }
        return count;
    }
    pub fn getMoves(&self, x : usize, y : usize) -> [[bool;9];10] {
        let mut flagBoard :  [[bool;9];10] = Default::default();
        let piece : &Piece = self.squares[y][x].piece.as_ref().unwrap();
        match piece.pieceType { 
            PieceType::Pawn => {
                if piece.isRed {
                    //forward
                    if y != BLACK_ROW {
                        self.TryMove(x, y+1, piece.isRed, &mut flagBoard);
                    }
                    //sideways
                    if y >= BLACK_RIVER {
                        if x > 0 {
                            self.TryMove(x-1, y, piece.isRed, &mut flagBoard);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y, piece.isRed, &mut flagBoard);
                        }
                    }

                } else {
                    //forward
                    if y != RED_ROW {
                        self.TryMove(x, y-1, piece.isRed, &mut flagBoard);
                    }
                    //sideways
                    if y <= RED_RIVER {
                        if x > 0 {
                            self.TryMove(x-1, y, piece.isRed, &mut flagBoard);
                        }
                        if x < 8 {
                            self.TryMove(x+1, y, piece.isRed, &mut flagBoard);
                        }
                    }
                }
            }
            PieceType::Advisor => {
                //bounds checking is more lax since advisors can only bump the top & bottom borders, not left & right
                if y != BLACK_ROW {
                    //up & left
                    if BoardState::IsPalace(x-1, y+1) {
                        self.TryMove(x-1, y+1, piece.isRed, &mut flagBoard);
                    }
                    //up & right
                    if BoardState::IsPalace(x+1, y+1) {
                        self.TryMove(x+1, y+1, piece.isRed, &mut flagBoard);
                    }
                }
                if y != RED_ROW {
                    //down & left
                    if BoardState::IsPalace(x-1, y-1) {
                        self.TryMove(x-1, y-1, piece.isRed, &mut flagBoard);
                    }
                    //down & right
                    if BoardState::IsPalace(x+1, y-1) {
                        self.TryMove(x+1, y-1, piece.isRed, &mut flagBoard);
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
                        self.TryMove(0, 2, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 2, piece.isRed, &mut flagBoard);
                    }
                    (6,0) => {
                        self.TryMove(8, 2, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 2, piece.isRed, &mut flagBoard);
                    }
                    (0,2) => {
                        self.TryMove(2, 4, piece.isRed, &mut flagBoard);
                        self.TryMove(2, 0, piece.isRed, &mut flagBoard);
                    },
                    (4,2) => {
                        self.TryMove(2, 4, piece.isRed, &mut flagBoard); //up and left
                        self.TryMove(6, 4, piece.isRed, &mut flagBoard); //up and right
                        self.TryMove(2, 0, piece.isRed, &mut flagBoard); //down and left
                        self.TryMove(6, 0, piece.isRed, &mut flagBoard); //down and right
                    },
                    (8,2) => {
                        self.TryMove(6, 4, piece.isRed, &mut flagBoard);
                        self.TryMove(6, 0, piece.isRed, &mut flagBoard);
                    },
                    (2,4) => {
                        self.TryMove(0, 2, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 2, piece.isRed, &mut flagBoard);
                    },
                    (6,4) => {
                        self.TryMove(4, 2, piece.isRed, &mut flagBoard);
                        self.TryMove(8, 2, piece.isRed, &mut flagBoard);
                    },
                    //BLACK ELEPHANTS
                    (2,9) => {
                        self.TryMove(0, 7, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 7, piece.isRed, &mut flagBoard);
                    }
                    (6,9) => {
                        self.TryMove(8, 7, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 7, piece.isRed, &mut flagBoard);
                    }
                    (0,7) => {
                        self.TryMove(2, 5, piece.isRed, &mut flagBoard);
                        self.TryMove(2, 9, piece.isRed, &mut flagBoard);
                    },
                    (4,7) => {
                        self.TryMove(2, 5, piece.isRed, &mut flagBoard); //up and left
                        self.TryMove(6, 5, piece.isRed, &mut flagBoard); //up and right
                        self.TryMove(2, 9, piece.isRed, &mut flagBoard); //down and left
                        self.TryMove(6, 9, piece.isRed, &mut flagBoard); //down and right
                    },
                    (8,7) => {
                        self.TryMove(6, 5, piece.isRed, &mut flagBoard);
                        self.TryMove(6, 9, piece.isRed, &mut flagBoard);
                    },
                    (2,5) => {
                        self.TryMove(0, 7, piece.isRed, &mut flagBoard);
                        self.TryMove(4, 7, piece.isRed, &mut flagBoard);
                    },
                    (6,5) => {
                        self.TryMove(4, 7, piece.isRed, &mut flagBoard);
                        self.TryMove(8, 7, piece.isRed, &mut flagBoard);
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
                    if self.squares[y+1][x].piece.is_none() { // Knights can be blocked in Xiangqi!
                        self.TryMove(x-1, y+2, piece.isRed, &mut flagBoard);
                        self.TryMove(x+1, y+2, piece.isRed, &mut flagBoard);
                    }
                }
                //left
                if x > 1 {
                    if self.squares[y][x-1].piece.is_none() {
                        self.TryMove(x-2, y+1, piece.isRed, &mut flagBoard);
                        self.TryMove(x-2, y-1, piece.isRed, &mut flagBoard);
                    }
                }
                //down
                if y > RED_ROW + 1 {
                    if self.squares[y-1][x].piece.is_none() {
                        self.TryMove(x-1, y-2, piece.isRed, &mut flagBoard);
                        self.TryMove(x+1, y-2, piece.isRed, &mut flagBoard);
                    }
                }
                //right
                if x < 7 {
                    if self.squares[y][x+1].piece.is_none() {
                        self.TryMove(x+2, y+1, piece.isRed, &mut flagBoard);
                        self.TryMove(x+2, y-1, piece.isRed, &mut flagBoard);
                    }
                }
            }
            PieceType::Cannon => {
                for ray in self.GetRaysFrom(x,y) {
                    let mut foundHoppable = false;
                    for (pos, tile) in ray {
                        if foundHoppable {
                            if tile.piece.is_none() {
                                continue;
                            }
                            self.TryMove(pos.0,pos.1, piece.isRed, &mut flagBoard);
                            break;
                        } else {
                            if tile.piece.is_none() {
                                self.TryMove(pos.0, pos.1, piece.isRed, &mut flagBoard);
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
                        self.TryMove(pos.0, pos.1, piece.isRed, &mut flagBoard);   
                        if tile.piece.is_some() {
                            break;
                        }
                    }
                }
            }
            PieceType::King => {
                //up
                if y < BLACK_ROW && BoardState::IsPalace(x, y+1) {
                    self.TryMove(x, y+1, piece.isRed, &mut flagBoard);  
                }
                //left
                if x > 0 && BoardState::IsPalace(x-1, y) {
                    self.TryMove(x-1, y, piece.isRed, &mut flagBoard);  
                }
                //down
                if y > RED_ROW && BoardState::IsPalace(x, y-1) {
                    self.TryMove(x, y-1, piece.isRed, &mut flagBoard);  
                }
                //right
                if x < 8 && BoardState::IsPalace(x+1, y) {
                    self.TryMove(x+1, y, piece.isRed, &mut flagBoard);  
                }
            }
        }

        return flagBoard;
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getChar())
    }
}
