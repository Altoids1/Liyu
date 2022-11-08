use super::DEAD_PIECE_COORD;
use super::packedmove::{PackedCoord,DEAD_PIECE_PACKEDCOORD};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Advisor,
    Elephant,
    Horse,
    Cannon,
    Rook,
    King
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Piece
{
    pub pieceType : PieceType,
    pub isRed : bool,
    pub loc : (usize,usize)
}

impl Piece {
    pub fn new(cara : char, newLocation : (usize,usize)) -> Self {
        match cara {
            'p' => {
                return Piece {pieceType : PieceType::Pawn, isRed : false, loc : newLocation};
            },
            'P' => {
                return Piece {pieceType : PieceType::Pawn, isRed : true, loc : newLocation};
            },
            'a' => {
                return Piece {pieceType : PieceType::Advisor, isRed : false, loc : newLocation};
            },
            'A' => {
                return Piece {pieceType : PieceType::Advisor, isRed : true, loc : newLocation};
            },
            'e' => {
                return Piece {pieceType : PieceType::Elephant, isRed : false, loc : newLocation};
            },
            'E' => {
                return Piece {pieceType : PieceType::Elephant, isRed : true, loc : newLocation};
            },
            'h' | 'n' => {
                return Piece {pieceType : PieceType::Horse, isRed : false, loc : newLocation};
            },
            'H' | 'N' => {
                return Piece {pieceType : PieceType::Horse, isRed : true, loc : newLocation};
            },
            'c' => {
                return Piece {pieceType : PieceType::Cannon, isRed : false, loc : newLocation};
            },
            'C' => {
                return Piece {pieceType : PieceType::Cannon, isRed : true, loc : newLocation};
            },
            'r' => {
                return Piece {pieceType : PieceType::Rook, isRed : false, loc : newLocation};
            },
            'R' => {
                return Piece {pieceType : PieceType::Rook, isRed : true, loc : newLocation};
            },
            'k' => {
                return Piece {pieceType : PieceType::King, isRed : false, loc : newLocation};
            },
            'K' => {
                return Piece {pieceType : PieceType::King, isRed : true, loc : newLocation};
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

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.getChar())
    }
}



#[derive(Clone, PartialEq, Eq, Hash)]
/// Barebones piecedata holder; only holding their coords. Their type & colour are implied by position & which PieceSet is used.
pub(crate) struct PieceSet {
    pub King : PackedCoord,
    pub Rooks : [PackedCoord;2],
    pub Cannons : [PackedCoord;2],
    pub Horses : [PackedCoord;2],
    pub Elephants : [PackedCoord;2],
    pub Advisors : [PackedCoord;2],
    pub Pawns : [PackedCoord;5]
}

impl Default for PieceSet {
    fn default() -> Self {
        return Self {
            King: DEAD_PIECE_PACKEDCOORD,
            Rooks: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD],
            Cannons: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD],
            Horses: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD],
            Elephants: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD],
            Advisors: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD],
            Pawns: [DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD,DEAD_PIECE_PACKEDCOORD]
        };
    }
}

pub(crate) struct PieceSetIterator<'a> {
    index : usize,
    isRed : bool,
    setRef : &'a PieceSet
}

impl<'a> PieceSetIterator<'a> {
    pub fn new(set : &'a PieceSet, is_red : bool) -> Self {
        return Self {
            index : 0,
            isRed : is_red,
            setRef : set
        };
    }
}


impl<'a> Iterator for PieceSetIterator<'a> {
    type Item = Piece;
    fn next(&mut self) -> Option<Self::Item> {
         let mut ret: Piece = match self.index {
            0 => Piece::new('r',self.setRef.Rooks[0].makeCoord()),
            1 => Piece::new('r',self.setRef.Rooks[1].makeCoord()),
            2 => Piece::new('c',self.setRef.Cannons[0].makeCoord()),
            3 => Piece::new('c',self.setRef.Cannons[1].makeCoord()),
            4 => Piece::new('h',self.setRef.Horses[0].makeCoord()),
            5 => Piece::new('h',self.setRef.Horses[1].makeCoord()),
            6 => Piece::new('e',self.setRef.Elephants[0].makeCoord()),
            7 => Piece::new('e',self.setRef.Elephants[1].makeCoord()),
            8..=12 => Piece::new('p',self.setRef.Pawns[self.index - 8usize].makeCoord()),
            13 => Piece::new('k',self.setRef.King.makeCoord()),
            14 => Piece::new('a',self.setRef.Advisors[0].makeCoord()),
            15 => Piece::new('a',self.setRef.Advisors[1].makeCoord()),
            _ => return None
        };
        if ret.loc == DEAD_PIECE_COORD {
            self.index +=1;
            return self.next(); // Bad to do it this way but it is the cleanest
        }
        ret.isRed = self.isRed;
        self.index +=1;
        return Some(ret);
    }
    
}