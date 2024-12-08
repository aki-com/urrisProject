
use std::convert::TryInto;
mod process_read;


pub struct Data {
    pub field_ptr: usize,
    pub app_option: process_read::ProcessRead
}


impl Data {
    pub fn field_read16(&self) -> [[u32; 16]; 20] {
        let mut field_raw: [u32; 512] = [0; 512];
        let mut field: [[u32; 16]; 20] = [[0; 16]; 20];
        self.app_option.read_memory::<[u32; 512]>(&mut field_raw, self.field_ptr).unwrap();
        println!("{:X}", self.field_ptr);
        for i in 0..20 {
            let row = &field_raw[i*16..(i+1)*16];
            field[i] = row.try_into().unwrap();
        }   
        field

    }

    
}

pub fn new() -> Data {
    let option = process_read::new("Tetris_UI.exe");
    //let offsets = vec![0x007E9000, 0x0, 0xE0, 0x1F0, 0x58, 0x0];//自ミノ込み
    let offsets = vec![0x007E3960,0x40,0x20,0x8,0x18,0x0,0x0];
    let address0 = option.read_memory_chain(&offsets).unwrap();

    Data {
        field_ptr: address0,
        app_option: option
    }
}
