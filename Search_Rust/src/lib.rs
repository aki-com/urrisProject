#![feature(portable_simd)]
mod tetris;
mod mino_def;
use mino_def::PlayData;
use numpy::PyArray2;
use pyo3::prelude::*;
use std::simd::u16x32;
use numpy::PyArrayMethods;
use std::sync::{Arc, Mutex};


#[pyclass]
struct SearchRust {
    
    player: Arc<tetris::Player>,
    now_field: Arc<Mutex<PlayData>>,

}
#[pymethods]
impl SearchRust {
    #[new]
    fn new() -> Self {
        let player = tetris::Player::new(0);
        SearchRust { 
            player: Arc::new(player),
            now_field: Arc::new(Mutex::new(PlayData::new())),
         }
    }
    fn read(&mut self) -> () {
        self.now_field = Arc::new(Mutex::new(self.player.read()));

    }
    /*fn search(&self, py: Python) -> Py<PyArray2<u16>> {
        let search_result: heapless::Vec<u16x32, 256> = self.player.search(self.now_field);
        let arr = PyArray2::zeros(py, (search_result.len(), 32), false);
        for (i, r) in search_result.iter().enumerate() {
            let row = r.to_array();
            for (j, value) in row.iter().enumerate() {
                unsafe {
                    *arr.get_mut([i, j]).unwrap() = *value;
                }
            }
        }
        PyArray2::from_array(py, &arr.to_owned_array()).into()
    }*/
}
unsafe impl Send for SearchRust {}
unsafe impl Sync for SearchRust {}





#[pymodule(gil_used = false)]
fn search_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SearchRust>()?;
    Ok(())
}
