use std::mem::transmute;
use std::fmt::{Binary,Error,Formatter,Display,Debug};
use std::cmp::Ordering;

/// So The Deal here is that uh
/// we want the engine to be able to pass around just, basic floats to describe the current evaluation of the board
/// however, sometimes the evaluation amounts to things like "black has won" or "the position is illegal"
/// in which case, we need to return something else!
/// The solution, for now, is to do NaN boxing, as described (for example) here: http://www.craftinginterpreters.com/optimization.html#what-is-and-is-not-a-number
#[derive(Clone)]
pub struct ScoreF32 {
    pub data : f32
}

const NAN_BASE : u32 = 0b0_11111111_100_00000_00000_00000_00000u32;

//A mask to apply over the NAN_BASE to indicate a particular genre of information.
#[repr(u32)]
enum ScoreMasks {
    RedWon =        0b0_00000000_010_00000_00000_00000_00000u32,
    BlackWon =      0b0_00000000_001_00000_00000_00000_00000u32,
    RedMating =     0b1_00000000_010_00000_00000_00000_00000u32,
    BlackMating =   0b1_00000000_001_00000_00000_00000_00000u32,
    InvalidPos =    0b1_00000000_011_00000_00000_00000_00000u32
}

///<remarks>
/// So, like, this COULD HAVE used INFINITY and NEG_INFINITY to describe these values
/// However, doing it this way allows for a distinction between "a floating-point overflow occurred doing evaluation" and "the engine confirmed victory is unavoidable"
///</remarks>
pub const RED_WON : ScoreF32 = ScoreF32::new_from_binary(NAN_BASE | ScoreMasks::RedWon as u32);
pub const BLACK_WON : ScoreF32 = ScoreF32::new_from_binary(NAN_BASE | ScoreMasks::BlackWon as u32);
pub const INVALID_POS : ScoreF32 = ScoreF32::new_from_binary(NAN_BASE | ScoreMasks::InvalidPos as u32);

impl ScoreF32 {
    pub const fn new(val : f32) -> Self {
        return Self { data : val };
    }
    pub const fn new_from_binary(val : u32) -> Self {
        unsafe { return Self { data: transmute::<u32,f32>(val)}; }
    }
    const fn to_bytes(&self) -> u32 {
        unsafe { return transmute::<f32,u32>(self.data); }
    }
}

impl Default for ScoreF32 {
    fn default() -> Self {
        return Self::new(0f32);
    }
}

#[allow(non_upper_case_globals)]
impl Debug for ScoreF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        let my_bytes = self.to_bytes();
        const invalid_bytes : u32 = INVALID_POS.to_bytes();
        const redwon_bytes : u32 = RED_WON.to_bytes();
        const blackwon_bytes : u32 = BLACK_WON.to_bytes();
        match my_bytes {
            invalid_bytes => write!(f,"INVALID_POS"),
            redwon_bytes => write!(f,"RED_WON"),
            blackwon_bytes => write!(f,"BLACK_WON"),
            _ => write!(f,"{}",self.data)
        }
    }
}

impl PartialEq for ScoreF32 {
    fn eq(&self, other: &Self) -> bool { // Must be PERFECT bitwise equality
        unsafe {
            return transmute::<f32,u32>(self.data) == transmute::<f32,u32>(other.data);
        }
    }
}
impl<'a> PartialEq<&'a ScoreF32> for ScoreF32 {
    fn eq(&self, other: &&'a ScoreF32) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<ScoreF32> for &'a ScoreF32 {
    fn eq(&self, other: &ScoreF32) -> bool {
        *self == other
    }
}

impl Eq for ScoreF32 {}

impl Display for ScoreF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /* TODO: Try to make this code work. WTF is match's problem here, throwing because this isn't a const expression?
        return match self {
            &RED_WON => write!(f, "Red Wins"),
            &BLACK_WON => write!(f, "Black Wins"),
            &INVALID_POS => write!(f, "Invalid position"),
            _ => write!(f,"{}",self.data)
        };
        */
        if self == RED_WON {
            return write!(f, "Red Wins");
        }
        if self == BLACK_WON {
            return write!(f, "Black Wins");
        }
        if self == INVALID_POS {
            return write!(f, "Invalid position");
        }
        return write!(f,"{}",self.data);
    }
}

impl Binary for ScoreF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Just uses u32's thing
        Binary::fmt( unsafe { &transmute::<f32, u32>(self.data)}, f)
    }
}

///Can't implement Ord because INVALID_POS has no reasonable ordering
#[allow(non_upper_case_globals)] // go frick yourself Rust, you got me into this mess in the first place
impl PartialOrd for ScoreF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { // ordering should be "$self is [blah] to $other"
        //Weird shitty kludginess here to allow us to use match,
        //since it erroneously lacks the functionality of supporting user-defined equality.
        let my_bytes = self.to_bytes();
        let their_bytes = other.to_bytes();
        const invalid_bytes : u32 = INVALID_POS.to_bytes();
        const redwon_bytes : u32 = RED_WON.to_bytes();
        const blackwon_bytes : u32 = BLACK_WON.to_bytes();

        match my_bytes {
            invalid_bytes => {
                return None; // WARNING: This is a null != null situation.
            }
            redwon_bytes => {
                match their_bytes {
                    invalid_bytes => {
                        return None;
                    }
                    redwon_bytes => {
                        return Some(Ordering::Equal);
                    }
                    _ => { // This actually does mean (intentionally) that RED_WON is greater than INF.
                        return Some(Ordering::Greater);
                    }
                }
            }
            blackwon_bytes => {
                match their_bytes {
                    invalid_bytes => {
                        return None;
                    }
                    blackwon_bytes => {
                        return Some(Ordering::Equal);
                    }
                    _ => {
                        return Some(Ordering::Less);
                    }
                }
            }
            _ => {
                match their_bytes {
                    invalid_bytes => {
                        return None;
                    }
                    redwon_bytes => {
                        return Some(Ordering::Less);
                    }
                    blackwon_bytes => {
                        return Some(Ordering::Greater);
                    }
                    _ => {
                        return self.data.partial_cmp(&other.data);
                    }
                }
            }
        };
    }
}