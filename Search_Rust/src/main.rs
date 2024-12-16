
#![feature(portable_simd)]
mod tetris;
mod mino_def;

fn main() {
    
    
    let player = tetris::Player::new(0);
  
    let data = player.read();


   
    let search_result = player.bfssearch(data);
    for i in 0..20 {
        println!("{:16b}", search_result[i]);
    }


}
