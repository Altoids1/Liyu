use std::ops::Range;

use super::piece::Piece;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tile
{
    pub pieceIndex : Option<PieceIndex>
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PieceIndex {
    pub cara : char // SO DUMB
}

impl PieceIndex {
    pub fn new(c : char) -> Self {
        return Self {
            cara : c
        };
    }
    pub fn asChar(&self) -> char {
        return self.cara;
    }
}

///Iterator used to iterate over the tiles of a BoardState.
pub struct TileIterator<'a> {
    squaresRef : &'a [[Tile;9];10],
    x : usize,
    y : usize,
    doneFirst : bool // SO DUMB
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
            pieceIndex: Default::default()
        }
    }
}

impl<'a> TileIterator<'a> {
    pub fn new(squares : &'a[[Tile;9];10]) -> Self {
        return Self {
            squaresRef : squares,
            x : 0,
            y : 0,
            doneFirst : false
        };
    }
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = ((usize,usize),&'a Tile);
    fn next(&mut self) -> Option<Self::Item> { // FIXME: Make this not suck.
        if !self.doneFirst {
            let ret = Some(((self.x,self.y),&self.squaresRef[self.y][self.x]));
            self.doneFirst = true;
            return ret;
        }
        if self.x < 8 {
            self.x += 1;
        }
        else if self.y < 9 {
            self.x = 0;
            self.y += 1;
        } else {
            return None;
        }
        return Some(((self.x,self.y),&self.squaresRef[self.y][self.x]));
    }
}