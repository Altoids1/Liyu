use super::TileGrid;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tile
{
    pub pieceIndex : PieceIndex
}

const NO_PIECE : PieceIndex = PieceIndex::new('\0');

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PieceIndex {
    pub cara : char // SO DUMB
}

impl PieceIndex {
    pub const fn new(c : char) -> Self {
        return Self {
            cara : c
        };
    }
    pub const fn asChar(&self) -> char {
        return self.cara;
    }
}

///Iterator used to iterate over the tiles of a BoardState.
pub struct TileIterator<'a> {
    squaresRef : &'a TileGrid,
    x : usize,
    y : usize,
    doneFirst : bool // SO DUMB
}

impl Tile {
    #[allow(dead_code)] // things need constructors, Rust, god
    pub const fn new() -> Self {
        return Tile::real_default_because_traits_suck();
    }

    ///TODO: Get rid of this when they allow functions to be const in traits.
    const fn real_default_because_traits_suck() -> Self {
        return Self {
            pieceIndex: NO_PIECE
        }
    }

    pub const fn hasPiece(&self) -> bool {
        return self.pieceIndex.cara == '\0';
    }
    
    // FIXME:
    //  mutable references are not allowed in constant functions
    //  see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information 
    pub fn take(&mut self) -> PieceIndex {
        let ret = self.pieceIndex.clone();
        self.pieceIndex = NO_PIECE;
        return ret;
    }
}

impl Default for Tile {
    fn default() -> Self {
        return Tile::real_default_because_traits_suck();
    }
}

impl<'a> TileIterator<'a> {
    pub fn new(squares : &'a TileGrid) -> Self {
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