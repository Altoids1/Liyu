//! Is all the information necessary to define a particular state of the board.

pub struct BoardState
{
    // first dimension is x (a to i), second is y (1 to 10)
    squares : [[Tile;9];10],
    isRedTurn : bool,
    plyNumber : i32, // One-indexed. Either player moving increments this. Odd for Red and even for Black
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

//

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
    pub fn loadFEN(&mut self, fenStr : &str) {
        let mut x : usize = 0;
        let mut y : usize = 9;
        for cara in fenStr.chars() {
            if cara == '/' {
                y -= 1;
                x = 0;
                continue;
            }
            if cara.is_numeric() {
                x += cara.to_digit(10).unwrap_or(1) as usize;
                continue;
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
    }

    pub fn Display(&self) {
        print!("Position value: {}\n",self.getValue());
        for arr in self.squares.iter() {
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
        for arr in self.squares.iter() { // TODO: Make this more complicated than just piece value summing
            for tile in arr {
                if tile.piece.is_none() {
                    continue;
                }
                let piece : &Piece = &tile.piece.as_ref().unwrap();
                let mut ourVal: f32 = match(piece.pieceType) {
                    PieceType::Pawn => 1f32, // TODO: Increase value after they cross the river
                    PieceType::Advisor => 2f32,
                    PieceType::Elephant => 2f32,
                    PieceType::Horse => 4f32,
                    PieceType::Cannon => 4.5f32,
                    PieceType::Rook => 9f32,
                    PieceType::King => 0f32 // We treat this differently :3
                };
                if !piece.isRed {
                    ourVal *= -1f32;
                }
                sum += ourVal;
            }
        }
        return sum;
    }
}

impl Tile {
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