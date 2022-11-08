use crate::board::{TileGrid, tile::Tile};

use super::{Coord,DEAD_PIECE_COORD};

/// We need 16 bits to store a move.
/// 0-4   : x of starting Coord
/// 5-8   : y of starting Coord
/// 9-12  : x of ending Coord
/// 12-15 : y of ending Coord
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PackedMove {
    pub data : u16
}

impl PackedMove {
    pub fn new_from_Coords(movePair : (Coord,Coord)) -> Self {
        debug_assert!(movePair.0 != movePair.1);
        let mut datum : u16 = 0;
        datum |= ((movePair.0.0 as u16) & 0b1111) << 12;
        datum |= ((movePair.0.1 as u16) & 0b1111) << 8;
        datum |= ((movePair.1.0 as u16) & 0b1111) << 4;
        datum |= (movePair.1.1 as u16) & 0b1111;
        return Self {
            data : datum
        };
    }
    fn getLetter(x_val : u16) -> char {
        const LETTERS : &'static [u8] = "abcdefghij".as_bytes();
        return LETTERS[x_val as usize] as char;
    }

    ///Indexes for the starting coordinate.
    pub fn indexStart<'a>(arr : &'a TileGrid, ind : &PackedMove) -> &'a Tile {
        return &arr
            [((ind.data & 0b0000_1111_0000_0000) >> 8u16) as usize]
            [((ind.data & 0b1111_0000_0000_0000) >> 12u16) as usize];
    }
    ///Indexes for the starting coordinate, mutably.
    pub fn indexStartMut<'a>(arr : &'a mut TileGrid, ind : &PackedMove) -> &'a mut Tile {
        return &mut arr
            [((ind.data & 0b0000_1111_0000_0000) >> 8u16) as usize]
            [((ind.data & 0b1111_0000_0000_0000) >> 12u16) as usize];
    }

    ///Indexes for the ending coordinate.
    pub fn indexEnd<'a>(arr : &'a TileGrid, ind : &PackedMove) -> &'a Tile {
        return &arr
            [(ind.data & 0b1111) as usize]
            [((ind.data & 0b0000_0000_1111_0000) >> 4u16) as usize];
    }
    ///Indexes for the ending coordinate, mutably.
    pub fn indexEndMut<'a>(arr : &'a mut TileGrid, ind : &PackedMove) -> &'a mut Tile {
        return &mut arr
            [(ind.data & 0b1111) as usize]
            [((ind.data & 0b0000_0000_1111_0000) >> 4u16) as usize];
    }

    pub fn start(&self) -> PackedCoord {
        return PackedCoord { data: ((self.data & 0b1111_1111_0000_0000) >> 8u16) as u8 };
    }
    pub fn end(&self) -> PackedCoord {
        return PackedCoord { data: (self.data & 0b1111_1111) as u8 };
    }

    pub const fn killsPiece(&self) -> bool { // True if the piece moves to an encoded DEAD_PIECE_COORD
        return (self.data & 0b0000_0000_1111_1111u16) == 0b1111_1111u16;
    }
}

impl std::fmt::Display for PackedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_start = (self.data & 0b1111_0000_0000_0000) >> 12u16;
        let y_start = (self.data & 0b0000_1111_0000_0000) >> 8u16;
        let x_end = (self.data & 0b0000_0000_1111_0000) >> 4u16;
        let y_end = self.data & 0b0000_0000_0000_1111;
        write!(f, "{}{}{}{}", Self::getLetter(y_start),x_start+1,Self::getLetter(y_end),x_end+1)
    }
}

/// For contexts where we REALLY need to be this careful about memory. <br/>
/// 0-4   : x of Coord <br/>
/// 5-8   : y of Coord
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PackedCoord {
    pub data : u8
}

pub const DEAD_PIECE_PACKEDCOORD : PackedCoord = PackedCoord::new_from_Coord(DEAD_PIECE_COORD);

impl PackedCoord {
    pub const fn new_from_Coord(coord : Coord) -> Self {
        return Self {
            data : (((coord.0 & 0b1111) << 4u16) | (coord.1 & 0b1111)) as u8
        };
    }

    ///Try to avoid doing this since the whole purpose of this struct is to reduce memory usage.
    pub fn makeCoord(&self) -> Coord {
        return (
            ((self.data & 0b1111_0000) >> 4u16) as usize,
            (self.data & 0b1111) as usize,
        );
    }
}