use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn matrix_multiply(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    console::log_1(&"matrix_multiply called!".into());
    
    let mut result = vec![0.0; n * n];
    
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
    
    result
}
