use super::Coord;

/// We need 16 bits to store a move.
/// 0-4   : x of starting Coord
/// 5-8   : y of starting Coord
/// 9-12  : x of ending Coord
/// 12-15 : y of ending Coord
pub struct PackedMove {
    data : u16
}

impl PackedMove {
    pub fn new_from_Coords(movePair : (Coord,Coord)) -> Self {
        debug_assert!(movePair.0 != movePair.1);
        let mut datum : u16 = 0;
        datum |= (movePair.0.0 as u16) & 0b1111 << 12;
        datum |= (movePair.0.1 as u16) & 0b1111 << 8;
        datum |= (movePair.1.0 as u16) & 0b1111 << 4;
        datum |= (movePair.1.1 as u16) & 0b1111;
        return Self {
            data : datum
        };
    }
    fn getLetter(x_val : u16) -> char {
        const LETTERS : &'static [u8] = "abcdefghij".as_bytes();
        return LETTERS[x_val as usize] as char;
    }
}

impl std::fmt::Display for PackedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_start = (self.data & 0b1111_0000_0000_0000) >> 12;
        let y_start = (self.data & 0b0000_1111_0000_0000) >> 8;
        let x_end = (self.data & 0b0000_0000_1111_0000) >> 4;
        let y_end = self.data & 0b0000_0000_0000_1111;
        write!(f, "{}{}{}{}", Self::getLetter(x_start),y_start+1,Self::getLetter(x_end),y_end+1)
    }
}
