use vstd::prelude::*;
verus! {
fn vec_len<T>(x: &Vec<T>) -> (r: usize){
    x.len()
}
fn numpy_log10(x: Vec<f32>) -> (result: Vec<f32>){
    let n: usize = x.len();
    let mut result: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let xi = x[i];
        result.push(xi);
        i = i + 1;
    }
    result
}
}
fn main() {}