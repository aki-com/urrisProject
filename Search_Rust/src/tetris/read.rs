
use std::convert::TryInto;
mod process_read;

pub struct PtrData {
    pub field_ptr: usize,
    pub mino_ptr: usize,
    pub app_option: process_read::ProcessRead
}


impl PtrData {
    pub fn field_read16(&self) -> [[u32; 16]; 20] {
        let mut field_raw: [u32; 512] = [0; 512];
        let mut field: [[u32; 16]; 20] = [[0; 16]; 20];
        self.app_option.read_memory_list::<[u32; 512]>(&mut field_raw, self.field_ptr).unwrap();
        for i in 0..20 {
            let row = &field_raw[i*16..(i+1)*16];
            field[i] = row.try_into().unwrap();
        }   
        field

    }
    pub fn mino_read(&self) -> [u32; 4] {
        let mut mino_raw= [0; 4];
        self.app_option.read_memory_list::<[u32; 4]>(&mut mino_raw, self.mino_ptr).unwrap();
        mino_raw
    }


    pub fn new() -> PtrData {
        let app_option = process_read::new("Tetris_UI.exe");
        //let offsets = vec![0x007E9000, 0x0, 0xE0, 0x1F0, 0x58, 0x0];//自ミノ込み
        let field_offsets = vec![0x007E3960,0x40,0x20,0x8,0x18,0x0,0x0];
        let nimo_offsets = vec![0x007DFD40,0xCC8,0x38];
        let field_ptr = app_option.read_memory_chain(&field_offsets).unwrap();
        let mino_ptr = app_option.read_memory_chain(&nimo_offsets).unwrap()+0x660;
        

        PtrData {
            field_ptr,
            mino_ptr,
            app_option
        }
    }
}
