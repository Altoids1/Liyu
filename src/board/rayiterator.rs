use super::{TileGrid,Tile,Coord, BLACK_ROW, RED_ROW};

pub(crate) struct RayIterator<'a> {
    cardinal : usize,
    tileRef : &'a TileGrid,
    x : usize,
    y : usize,
    
}

impl<'a> RayIterator<'a> {
    pub fn new(set : &'a TileGrid, _x : usize, _y : usize) -> Self {
        return Self {
            cardinal : 0,
            tileRef : set,
            x : _x,
            y : _y
        };
    }
}
impl<'a> Iterator for RayIterator<'a> {
    type Item = Vec<(Coord, &'a Tile)>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret : Vec<(Coord, &'a Tile)>; // FIXME: Find some way to make this static without muh borrowing semantics complaining
        match self.cardinal {
            0 => { //go up
                ret = Vec::with_capacity(9);
                for i in self.y+1..=BLACK_ROW {
                    ret.push(((self.x,i),&self.tileRef[i][self.x]));
                }
            },
            1 => { // go left
                ret = Vec::with_capacity(9);
                for i in (0..self.x).rev() { // hate u rust
                    ret.push(((i,self.y),&self.tileRef[self.y][i]));
                }
            },
            2 => { // go right
                ret = Vec::with_capacity(9);
                for i in self.x+1..9 {
                    ret.push(((i,self.y),&self.tileRef[self.y][i]));
                }
            },
            3 => { // go down
                ret = Vec::with_capacity(9);
                for i in (RED_ROW..self.y).rev() {
                    ret.push(((self.x,i),&self.tileRef[i][self.x]));
                }
            },
            _ => {
                return None;
            }
        }
        self.cardinal += 1;
        return Some(ret);
    }
    
}