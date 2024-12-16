use crate::mino_def::{Mino, PlayData, MINO};

pub fn calculate_state(data:PlayData) -> [u16; 20] {
    let mut new_field: [u16; 20] = [0;20];
    add(data.field, MINO[data.mino.shape as usize][data.mino.angle as usize]);
    new_field
}


pub fn add(mino_f:[u16;20],state_f:[u16;20]) -> [u16; 20] {
    let mut new_field = [0;20];
    for i in 0..20 {
        new_field[i] = mino_f[i] | state_f[i];
    }
    new_field
}

pub struct State;
impl State {
    pub fn left(data:Mino) -> [u16; 20] {
        let mut new_mino = MINO[data.shape as usize][data.angle as usize];
        for i in 0..20 {
            new_mino[i] = new_mino[i] << data.x-6;
        }
        new_mino
    }
    pub fn right(data:Mino) -> [u16; 20] {
        let mut new_mino = MINO[data.shape as usize][data.angle as usize];
        for i in 0..20 {
            new_mino[i] = new_mino[i] >> data.x-6;
        }
        new_mino
    }
}