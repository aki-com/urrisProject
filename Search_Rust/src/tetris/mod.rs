use std::simd::{u32x16,prelude::*};

mod read;
mod search;
#[allow(dead_code)]
pub struct Player {
    number: u32,
    ptr_data: read::Data

    
}

impl Player {
    pub fn read(&self) -> [u16; 20] {
        let field = self.ptr_data.field_read16();
        field_bitmask16(field)
    }
    pub fn search(&self, field: [u16; 20]) -> [u16; 20] {
        let search = search::new(field, 0, [0;6]);
        search.search()
    }
    
}



pub fn new(number: u32) -> Player {

    Player {
        number,
        ptr_data: read::new()
    }
}

pub fn field_bitmask16(field: [[u32; 16]; 20]) -> [u16; 20] {
    let mut bitmask = [0u16; 20];

    for (row_idx, row) in field.iter().enumerate() {
        let row_simd= u32x16::from_array(*row);
        let mask = !row_simd.simd_eq(u32x16::splat(0));
        let mask = mask.to_bitmask();
        bitmask[row_idx] = mask as u16
    }

    bitmask
}