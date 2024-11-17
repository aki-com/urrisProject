mod app_read;


pub struct Player {
    number: u32,
    Field0Ptr: usize
}
impl Player {
    fn fieldRead
}
pub fn new(number: u32) -> Player {
    let Option = app_read::new("Tetris_UI.exe");
    let offsets = vec![0x007E9000, 0x0, 0xE0, 0x1F0, 0x58, 0x0];
    let address = Option.read_memory_chain(&offsets).unwrap();
    let value = 0;
    println!("Value: {}", value);
    Player {
        number: number,
        Field0Ptr: address
    }
}