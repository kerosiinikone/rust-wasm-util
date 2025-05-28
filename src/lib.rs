use std::{convert::TryInto, fmt};

use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub struct Matrix4 {
    matrix: [u32; 16],
    _len: u32
}

#[wasm_bindgen]
impl Matrix4 {
    fn new() -> Self {
        Matrix4 {
            matrix: [0; 16],
            _len: 16
        }
    }
}

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square = self._len.isqrt();
        for i in 1..square {
            for j in 1..square {
                let index: usize =  (j * i).try_into().unwrap();
                write!(f, "[{}]", self.matrix[index])?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub fn random_matrix4() -> Matrix4 {
    let mut rng = rand::rng();
    let mut m4 = Matrix4::new();

    for i in 1..16 {
        m4.matrix[i] = rng.random_range(1..100)
    }

    m4
}
