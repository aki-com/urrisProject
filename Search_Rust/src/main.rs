
#![feature(portable_simd)]
mod tetris;



fn main() {
    let player = tetris::new(1);
    let field = player.read();
    println!("{:?}", field.map(|val| format!("{:016b}", val)));
    let search_result = player.search(field);
    println!("{:?}", search_result.map(|val| format!("{:016b}", val)));
}
