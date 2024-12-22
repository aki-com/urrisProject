use mino_move::calculate_state;
use crate::mino_def::{Mino, PlayData};
use std::simd::u16x32;
use heapless::Vec;
use fxhash::{FxHashMap, FxBuildHasher};

mod mino_move;


pub struct Search{
    pub current: PlayData,
    pub first_field: u16x32,
    pub queue: Vec<Mino, 256>,
    pub result: Vec<u16x32, 256>,
    pub current_log:Vec<&'static str, 32>,
    pub unordered_map: FxHashMap<u16x32, Vec<&'static str, 32>>,
}
impl Search {
    pub fn new(data:PlayData) -> Self {
        let mut queue = Vec::new();
        let    result = Vec::new();
        let mut unordered_map = FxHashMap::with_capacity_and_hasher(256, FxBuildHasher::default());
        let mut move_log: Vec<&'static str, 32> = Vec::new();
        move_log.push("S").unwrap();
        let f = calculate_state(data).unwrap();
        unordered_map.insert(f, move_log.clone());
         queue.push(data.mino).unwrap();
         Search {
            current: data,
            first_field: data.field,
            queue,
            result,
            current_log: move_log,
            unordered_map,
        }
    }
    pub fn bfssearch(&mut self) -> Vec<u16x32,256> {
        loop {
            if !self.pop_curret() {
                break;
            }
          
            self.try_move("L");
            self.try_move("R");
            self.try_move("r");
            self.try_move("l");
            self.try_move("D");
            self.try_harddrop();
        }

        self.result.clone()
    }
    pub fn pop_curret(&mut self) -> bool {
        if let Some(m) = self.queue.pop() {
            self.current.mino = m;
            self.current.field = calculate_state(PlayData { field: self.first_field, mino: self.current.mino }).unwrap();
            self.current_log = self.unordered_map.get(&self.current.field).unwrap().clone();

            true
        }
        else {
            false
        }
    }
    pub fn try_move(&mut self, move_char:&'static str) {
        let mut new_state = self.current;
        match move_char {
            "R" => { new_state.mino.x += 1 }
            "L" => { new_state.mino.x -= 1 }
            "D" => { new_state.mino.y += 1 }
            "r" => { if new_state.mino.angle == 3 { new_state.mino.angle = 0 } else { new_state.mino.angle += 1 } }
            "l" => { if new_state.mino.angle == 0 { new_state.mino.angle = 3 } else { new_state.mino.angle -= 1 } }
            _ => {}
        }
        if let Some(field) = calculate_state(PlayData {field: self.first_field, mino: new_state.mino}) {
            if !self.unordered_map.contains_key(&field) {
                let mut move_log = self.current_log.clone();
                move_log.push(move_char).unwrap();
                self.unordered_map.insert(field, move_log);
                self.queue.push(new_state.mino).unwrap();
                //self.result.push(field).unwrap();
            }
                
        }
    }
    pub fn try_harddrop(&mut self) {
        let mut mino = self.current.mino;
        mino.y += 1;
        if None == calculate_state(PlayData {field: self.first_field, mino}) {
            self.result.push(self.current.field).unwrap();
        }


    }
}

