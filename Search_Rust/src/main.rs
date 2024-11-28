mod tetris_pointer;


fn main() {
    let player = tetris_pointer::new(1);
    let field = player.field_read();
    for element in &field {
        println!("aaaaa");
        println!("{:?}", element);
    }
    
}   


