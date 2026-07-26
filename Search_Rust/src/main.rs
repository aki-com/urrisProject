
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



    const vec_size: usize = 9;
    let mut search_results5: heapless::Vec<heapless::Vec<std::simd::Simd<u16, 32>, {vec_size}>, 256> = heapless::Vec::new();

    for (n, r) in search_result.iter().enumerate() {
        let k = n / vec_size;
        if n % vec_size == 0 {
            search_results5.push(heapless::Vec::new()).unwrap();
        }

        search_results5[k].push(*r).unwrap();
    }



    for r in search_results5.iter() {
        println!("---------------------------------");
        for i in 12..29 {
            for j in 0..r.len() {
                print!(" {:016b} ", r[j][i]);
            }   
            println!();
        }

    }
        println!("---------------------------------");
        
    }
