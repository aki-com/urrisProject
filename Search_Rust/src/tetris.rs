use std::time;
use std::simd::u16x32;
use crate::mino_def::{PlayData, Mino};
use heapless::Vec;
mod read;
mod search;

pub struct Player {
    number: u32,
    ptr_data: read::PtrData,
}




impl Player {
    pub fn new(number: u32) -> Player {
        
        Player {
            number,
            ptr_data: read::PtrData::new(),

        }
    }
    pub fn read(&self) -> PlayData {
        let field = self.ptr_data.field_read16();
        let mino = self.ptr_data.mino_read();
        PlayData {
            field,
            mino: Mino {
                shape: mino[0],
                x: mino[1],
                y: mino[2],
                angle: mino[3],
            }
        }
            
    }
    pub fn search(&self, mut data:PlayData) -> Vec<u16x32,256> {
      /* */
      println!("{:?}", data.mino);
      data.mino.y = 20;
        for (i,f) in data.field.to_array().iter().enumerate() {
            if *f > 57351 {
                //for終了
                
                println!("{:?}", i);
                break;  
            } 
            else {
               
            }
        }
        let mut search =search::Search::new(data);
        let time = time::Instant::now();

        for _ in 1..2 {
            search.bfssearch();
            
        }
        println!("time:{:?}", time.elapsed().as_secs_f64());
        search.result.clone()
    }
    
}

