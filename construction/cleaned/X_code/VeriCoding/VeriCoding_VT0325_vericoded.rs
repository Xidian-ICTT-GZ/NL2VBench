use vstd::prelude::*;
verus! {
fn imag(val: Vec<(f64, f64)>) -> (result: Vec<f64>){
    let mut result: Vec<f64> = Vec::new();
    let n: usize = val.len();
    let mut i: usize = 0;
    while i < n
    {
        let im = val[i].1;
        result.push(im);
        i = i + 1;
    }
    result
}
}
fn main() {}