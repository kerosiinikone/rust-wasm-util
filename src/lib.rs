use std::error::Error;
use std::fmt;
use std::result::Result;

use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub enum MatrixType {
    Four
}

pub trait Matrix {
    fn mtype(&self) -> &MatrixType;
    fn into_matrix4(&self) -> Matrix4 {
        Matrix4 {
            matrix: [[0; 4]; 4],
            _len: 16,
            _type: MatrixType::Four
        }
    }
   fn multiply(&self, m2: &Matrix4) -> Result<Matrix4, Box<(dyn Error)>> {
        match self.mtype() {
            MatrixType::Four => {
                Matrix4::multiply4(&self.into_matrix4(), m2)
            }
        }
   }
}

#[wasm_bindgen]
pub struct Matrix4 {
    matrix: [[u32; 4]; 4],
    _len: usize,
    _type: MatrixType
}

#[wasm_bindgen]
impl Matrix4 {
    /// Identity
    fn new() -> Self {
        Matrix4 {
            matrix: [[0; 4]; 4],
            _len: 16,
            _type: MatrixType::Four
        }
    }

    fn _iterate_matrix() {
        todo!()
    }
    
    /// Naive multiplication -> just trying to learn the Rust concepts
    fn multiply4(m1: &Matrix4, m2: &Matrix4) -> Result<Matrix4, Box<(dyn Error)>> {
        if m1._len != m2.matrix[0].len() {
            // Not of equal length
            return Err("m1 and m2 not of equal length".into());
        }

        let mut mult_matrix = Matrix4::new();

        let r1 = m1.matrix.len();
        let c1 = m1.matrix[0].len();
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

impl Matrix for Matrix4 {
   fn multiply(&self, m2: &Matrix4) -> Result<Matrix4, Box<(dyn Error)>> {
        Ok(Matrix4::multiply4(self, m2)?)
   }
   fn mtype(&self) -> &MatrixType {
       &self._type
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
