#![feature(portable_simd)]
use numpy::PyArray1;
use pyo3::{prelude::*, types::PyModule};
use std::simd::u16x32;
use pyo3::prelude::*;

mod tetris;

mod mino_def;



#[pymodule]
mod search_rust {
    use super::*;

    #[pyfunction]
    fn Search(py: Python) -> Py<PyArray1<u16>> {
        let player = tetris::Player::new(0);
        let data = player.read();
        let search_result = player.search(data);
        let flat_result: Vec<u16> = search_result
            .iter()
            .flat_map(|simd| simd.to_array())
            .collect();

        PyArray1::from_slice(py, &flat_result).into()
    }
}
