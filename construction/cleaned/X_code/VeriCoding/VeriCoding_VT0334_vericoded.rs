use vstd::prelude::*;
verus! {
fn logaddexp2_elem(a: f32, _b: f32) -> f32 {
    a
}
fn numpy_logaddexp2(x1: Vec<f32>, x2: Vec<f32>) -> (result: Vec<f32>){
    let n = x1.len();
    let mut res: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let v = logaddexp2_elem(x1[i], x2[i]);
        res.push(v);
        i += 1;
    }
    res
}
}
fn main() {}