use crate::mino_def::{PlayData, MINO};

pub fn calculate_state(data:PlayData) -> [u16; 20] {
    let mut mino_field: [u16; 20] = MINO[data.mino.shape as usize][data.mino.angle as usize];
    mino_field = right_and_left(mino_field, data.mino.x);
    mino_field = down(mino_field, data.mino.y);
    println!("{:?}",data.mino);
    add(data.field, mino_field)
}


pub fn add(mino_f:[u16;20],state_f:[u16;20]) -> [u16; 20] {
    let mut new_field = [0;20];
    for i in 0..20 {
        new_field[i] = mino_f[i] | state_f[i];
    }
    new_field
}

pub fn right_and_left(mino_f: [u16; 20], x: u32) -> [u16; 20] {
    let move_x = 6 - x as i8;
    if move_x == 0 {
        return mino_f;
    }

    let mut new_mino = mino_f;
    if move_x > 0 {
        for value in &mut new_mino {
            *value <<= move_x as u8;
        }
    } else {
        for value in &mut new_mino {
            *value >>= -move_x as u8;
        }
    }
    new_mino
}

pub fn down(mino_f:[u16;20],y:u32) -> [u16; 20] {
    let mut new_mino = mino_f;
    new_mino.rotate_right((y-1) as usize);
    new_mino
}
