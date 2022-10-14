pub enum PieceType {
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
    pub pieceType : PieceType,
    pub isRed : bool
}

impl Piece {
    pub fn new(cara : char) -> Self {
        match cara {
            'p' => {
                return Piece {pieceType : PieceType::Pawn, isRed : false};
            },
            'P' => {
                return Piece {pieceType : PieceType::Pawn, isRed : true};
            },
            'a' => {
                return Piece {pieceType : PieceType::Advisor, isRed : false};
            },
            'A' => {
                return Piece {pieceType : PieceType::Advisor, isRed : true};
            },
            'e' => {
                return Piece {pieceType : PieceType::Elephant, isRed : false};
            },
            'E' => {
                return Piece {pieceType : PieceType::Elephant, isRed : true};
            },
            'h' => {
                return Piece {pieceType : PieceType::Horse, isRed : false};
            },
            'H' => {
                return Piece {pieceType : PieceType::Horse, isRed : true};
            },
            'c' => {
                return Piece {pieceType : PieceType::Cannon, isRed : false};
            },
            'C' => {
                return Piece {pieceType : PieceType::Cannon, isRed : true};
            },
            'r' => {
                return Piece {pieceType : PieceType::Rook, isRed : false};
            },
            'R' => {
                return Piece {pieceType : PieceType::Rook, isRed : true};
            },
            'k' => {
                return Piece {pieceType : PieceType::King, isRed : false};
            },
            'K' => {
                return Piece {pieceType : PieceType::King, isRed : true};
            },
            _ => {
                panic!("Attempted to create invalid piece '{cara}'");
            }
        }
    }
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
            character = character.to_ascii_uppercase();
        }
        return character;
    }
}
