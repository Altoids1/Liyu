use std::char::from_digit;

/// Is all the information necessary to define a particular state of the board.
pub struct BoardState
{
    // first dimension is x (a to i), second is y (1 to 10)
    squares : [[Tile;9];10],
    isRedTurn : bool,
    plyNumber : i32, // Zero-indexed. Either player moving increments this. Even for Red and odd for Black
}
pub struct Tile
{
    piece : Option<Piece>
}

enum PieceType {
    Pawn,
    Advisor,
    Elephant,
    Horse,
    Cannon,
    Rook,
    King
}
pub struct Piece
{
    pieceType : PieceType,
    isRed : bool
}

/// the Y index for where black's back rank is.
const BLACK_ROW : usize = 9;
/// the Y index for where red's back rank is.
const RED_ROW : usize = 0;
/// the Y index for where black's river starts.
const BLACK_RIVER : usize = 5;
/// the Y index for where red's river starts.
const RED_RIVER : usize = 4;

impl BoardState {
    pub fn new() -> Self {
        let mut ret =  Self {
            squares : Default::default(),
            isRedTurn : true,
            plyNumber : 1
        };

        ret.loadFEN("rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 1");

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
                'p' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Pawn, isRed : false});
                },
                'P' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Pawn, isRed : true});
                },
                'a' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Advisor, isRed : false});
                },
                'A' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Advisor, isRed : true});
                },
                'e' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Elephant, isRed : false});
                },
                'E' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Elephant, isRed : true});
                },
                'h' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Horse, isRed : false});
                },
                'H' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Horse, isRed : true});
                },
                'c' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Cannon, isRed : false});
                },
                'C' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Cannon, isRed : true});
                },
                'r' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Rook, isRed : false});
                },
                'R' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::Rook, isRed : true});
                },
                'k' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::King, isRed : false});
                },
                'K' => {
                    _ = self.squares[y][x].piece.insert(Piece {pieceType : PieceType::King, isRed : true});
                },

                ' ' => break, // TODO
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
                let mut cara : char = match piece.pieceType {
                    PieceType::Pawn => 'p',
                    PieceType::Advisor => 'a',
                    PieceType::Elephant => 'e',
                    PieceType::Horse => 'h',
                    PieceType::Cannon => 'c',
                    PieceType::Rook => 'r',
                    PieceType::King => 'k'
                };
                if piece.isRed { // Red is Capital!
                    cara = cara.to_ascii_uppercase();
                }
                fenString.push(cara);
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
            _ => {} // do nothing :#
        }

        return flagBoard;
    }
}

impl Tile {
    #[allow(dead_code)] // things need constructors, Rust, god
    pub fn new() -> Self {
        return Default::default();
    }
}

impl Default for Tile {
    fn default() -> Self {
        return Self {
            piece: Default::default()
        }
    }
}

impl Piece {
    pub fn getChar(&self) -> char {
        let mut character = match self.pieceType {
            PieceType::Pawn => 'p',
            PieceType::Advisor => 'a',
            PieceType::Elephant => 'e',
            PieceType::Horse => 'h',
            PieceType::Cannon => 'c',
            PieceType::Rook => 'r',
            PieceType::King => 'k'
        };
        if self.isRed { // red is uppercase, I've decided (goes with how chess FEN works)
            character = character.to_uppercase().next().unwrap_or(character);
        }
        return character;
    }
}