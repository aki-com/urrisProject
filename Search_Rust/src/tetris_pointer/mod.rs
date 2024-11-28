mod app_read;
use std::slice;

use read_process_memory::CopyAddress;


#[allow(dead_code)]
pub struct Player {
    number: u32,
    field_ptr: [usize; 20],
    option: app_read::ProcessRead
}
impl Player {
    pub fn field_read(&self) -> Vec<&[usize]> {
        let mut slices = Vec::new();

        for &ptr in &self.field_ptr {
            unsafe {
                let slice = slice::from_raw_parts(ptr as *const usize, 10); // 10個の要素
                slices.push(slice);
            }
        }
        slices
    }
}
pub fn new(number: u32) -> Player {
    let option = app_read::new("Tetris_UI.exe");
    let offsets = vec![0x007E9000, 0x0, 0xE0, 0x1F0, 0x58, 0x0];
    let address0 = option.read_memory_chain(&offsets).unwrap()+0x4CC;
    println!("{:X}", address0);
    let mut address :[usize; 20] = [0; 20];

    for (i, addr) in address.iter_mut().enumerate() {
        *addr = address0 + i * 0x4;
    }

    Player {
        number,
        field_ptr: address,
        option
    }
}