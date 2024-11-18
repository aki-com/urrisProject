mod app_read;


pub struct Player {
    number: u32,
    Field0Ptr: [usize; 20]
}
impl Player {
    fn fieldRead(&self) -> 
}
pub fn new(number: u32) -> Player {
    let Option = app_read::new("Tetris_UI.exe");
    let offsets = vec![0x007E9000, 0x0, 0xE0, 0x1F0, 0x58, 0x0];
    let address0 = Option.read_memory_chain(&offsets).unwrap()+0x40;
    let mut address :[usize; 20] = [0; 20];
    for i in 0..20 {
        address[i] = address0;
    }   

    Player {
        number: number,
        Field0Ptr: address
    }
}