use std::simd::{u32x16,prelude::*};
use crate::mino_def::{PlayData, Mino};
mod read;
mod search;
#[allow(dead_code)]
pub struct Player {
    number: u32,
    ptr_data: read::PtrData,
    play_data: PlayData

    
}




impl Player {
    pub fn new(number: u32) -> Player {
        
        Player {
            number,
            ptr_data: read::PtrData::new(),
            play_data: PlayData {
                field: [0; 20],
                mino: Mino {
                    shape: 0,
                    x: 0,
                    y: 0,
                    angle: 0
                }
            }
        }
    }
    pub fn read(&self) -> PlayData {
        let rew_field = self.ptr_data.field_read16();
        for i in 0..20 {
            println!("{:?}", rew_field[i]);
            
        }
        let field = field_bitmask16(rew_field);
        let mino = self.ptr_data.mino_read();
        PlayData {
            field,
            mino: Mino {
                shape: mino[0],
                x: mino[1],
                y: mino[2],
                angle: mino[3],
            }
        }
            
    }
    pub fn bfssearch(&self, data:PlayData) -> [u16; 20] {
        search::Search::search(data)
    }
    
}

pub fn field_bitmask16(field: [[u32; 16]; 20]) -> [u16; 20] {
    let mut bitmask = [0u16; 20];

    for (row_idx, row) in field.iter().enumerate() {
            let row_simd = u32x16::from_array(*row).reverse();
            let mask = !row_simd.simd_eq(u32x16::splat(0)).to_bitmask();
            bitmask[row_idx] = mask as u16;
        }

    bitmask
}