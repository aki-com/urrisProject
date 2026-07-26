
use std::simd::u16x32;
#[derive(Debug,Clone,Copy)]
pub struct PlayData {
   pub field: u16x32,
   pub mino: u8,
}
impl PlayData {
    pub fn new() -> PlayData {
        PlayData {
            field: u16x32::splat(0),
            mino: 0,
        }
    }
    
}