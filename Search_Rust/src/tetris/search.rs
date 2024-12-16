use mino_move::add;

use crate::mino_def::{PlayData, MINO};


mod mino_move;


pub struct Search;

impl Search {
    pub fn search(data:PlayData) -> [u16; 20] {
        add(MINO[data.mino.shape as usize][data.mino.angle as usize],data.field )
    }
}

