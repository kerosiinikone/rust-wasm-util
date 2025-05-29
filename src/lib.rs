use std::error::Error;
use std::fmt;
use std::result::Result;

use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub struct Matrix4 {
    matrix: [[u32; 4]; 4],
    _len: usize
}

#[wasm_bindgen]
impl Matrix4 {
    /// Identity
    fn new() -> Self {
        Matrix4 {
            matrix: [[0; 4]; 4],
            _len: 16
        }
    }

    fn _iterate_matrix() {
        todo!()
    }
    
    /// Naive multiplication -> just trying to learn the Rust concepts
    fn _multiply4(m1: &Matrix4, m2: &Matrix4) -> Result<Matrix4, Box<(dyn Error)>> {
        if m1._len != m2.matrix[0].len() {
            // Not of equal length
            return Err("m1 and m2 not of equal length".into());
        }

        let mut mult_matrix = Matrix4::new();

        let r1 = m1.matrix.len();
        let c1 = m1.matrix[0].len();
        let r2 = m2.matrix.len();
        let c2 = m2.matrix[0].len();

        for i in 0..r1 {
            for j in 0..c2 {
                for k in 0..c1 {
                    mult_matrix.matrix[i][j] += m1.matrix[i][k] * m2.matrix[k][j];
                }   
            }
        }

        Ok(mult_matrix)
    }
}

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let square = self._len.isqrt();
        for i in 0..square {
            for j in 0..square {
                write!(f, "[{}]", self.matrix[i][j])?
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

    let square = m4._len.isqrt();

    for i in 0..square {
        for j in 0..square {
            m4.matrix[i][j] = rng.random_range(1..100)
        }
    }

    m4
}
