use vstd::prelude::*;
verus! {
fn hermeline(off: f64, scl: f64) -> (coeffs: Vec<f64>){
    let mut coeffs_vec: Vec<f64> = Vec::new();
    coeffs_vec.push(off);
    coeffs_vec.push(scl);
    coeffs_vec
}
}
fn main() {}