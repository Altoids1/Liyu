use std::mem::transmute;
use std::fmt::{Binary,Error,Formatter};

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
}

impl Default for ScoreF32 {
    fn default() -> Self {
        return Self::new(0f32);
    }
}

impl Binary for ScoreF32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Just uses u32's thing
        Binary::fmt( unsafe { &transmute::<f32, u32>(self.data)}, f)
    }
}