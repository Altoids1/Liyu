#[derive(Clone)]
pub struct Tile
{
    pub piece : Option<crate::board::piece::Piece>
}

///Iterator used to iterate over the tiles of a BoardState.
pub struct TileIterator<'a> {
    squaresRef : &'a [[Tile;9];10],
    x : usize,
    y : usize
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

impl<'a> TileIterator<'a> {
    pub fn new(squares : &'a[[Tile;9];10]) -> Self {
        return Self {
            squaresRef : squares,
            x : 0,
            y : 0,
        };
    }
    fn increment(&mut self) -> bool {
        if self.x < 8 {
            self.x += 1;
            return true;
        }
        if self.y < 9 {
            self.y += 1;
            return true; 
        }
        return false;
    }
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = ((usize,usize),&'a Tile);
    fn next(&mut self) -> Option<Self::Item> {
        //handle indexing
        if !self.increment() {
            return None;
        }
        return Some(((self.x,self.y),&self.squaresRef[self.y][self.x]));
    }
}