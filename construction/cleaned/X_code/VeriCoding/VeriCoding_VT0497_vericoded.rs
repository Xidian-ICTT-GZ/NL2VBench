use vstd::prelude::*;
verus! {
fn legmulx(c: Vec<f32>) -> (result: Vec<f32>){
    let n = c.len();
    let mut result: Vec<f32> = Vec::new();
    result.push(0.0f32);
    let mut i: usize = 0;
    while i < n
    {
        result.push(c[i]);
        i = i + 1;
    }
    result
}
}
fn main() {}