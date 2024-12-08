
pub struct Search{
    field: [u16; 20],
    hold: u8,
    next: [u8;6]
}
pub fn new(field:[u16;20], hold:u8, next:[u8;6]) -> Search {
    Search{
        field,
        hold,
        next
    }
}
impl Search {
    pub fn search(&self) -> [u16; 20] {
        self.field
    }
}

