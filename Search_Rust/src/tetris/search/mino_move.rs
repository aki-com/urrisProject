use crate::mino_def::{PlayData, MINO};
use std::simd::{num::SimdUint, u16x32, Simd};

pub fn calculate_state(data:PlayData) -> Option<u16x32> {
    let mut mino_field = MINO[data.mino.shape as usize][data.mino.angle as usize];
    right_and_left(&mut mino_field, data.mino.x);
    down(&mut mino_field, data.mino.y);
    add(mino_field, data.field)
}

pub fn add(mino_f: u16x32, state_f: u16x32) -> Option<u16x32>{
    if 0 !=(mino_f & state_f).reduce_or() {
        None
    }
    else {
        Some(mino_f | state_f)
    }

}

pub fn right_and_left( mino_f: &mut Simd <u16,32>, x: u32){

    if x > 6 {
        *mino_f <<= Simd::splat(x as u16 - 6)
    } else {
        *mino_f >>= Simd::splat(6 - x as u16) 
    }
}

pub fn down(mino_f: &mut Simd<u16, 32>, y: u32){
    if y >= 10 {
        for _ in 0..y-10 {
            *mino_f = mino_f.rotate_elements_right::<1>();
        }

    }
}