use wasm_bindgen::prelude::*;
use rayon::prelude::*;

#[wasm_bindgen]
pub fn matrix_multiply(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut result = vec![0.0; n * n];

    result.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
        for k in 0..n {
            let a_ik = a[i * n + k];
            for j in 0..n {
                row[j] += a_ik * b[k * n + j];
            }
        }
    });

    result
}
