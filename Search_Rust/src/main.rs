
#![feature(portable_simd)]
mod tetris;
mod mino_def;
fn main() {
    
    let player = tetris::Player::new(0);
    let data = player.read();
    println!("{:?}", data.mino);
    println!("---------------------------------");        println!("---------------------------------");        println!("---------------------------------");
    let time = std::time::Instant::now();
    let search_result = player.search(data);
    println!("{:?}", time.elapsed());


for r in search_result.iter() {
        for i in 12..29 {
            println!("{:016b}", r[i]);
        }
        println!("---------------------------------");
        
    }
    
    println!("{:?}", search_result.len());
}