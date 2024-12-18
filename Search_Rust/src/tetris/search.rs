use mino_move::calculate_state;

use crate::mino_def::PlayData;


mod mino_move;


pub struct Search;

impl Search {
    pub fn search(data:PlayData) -> [u16; 20] {
        calculate_state(data)
    }
}

