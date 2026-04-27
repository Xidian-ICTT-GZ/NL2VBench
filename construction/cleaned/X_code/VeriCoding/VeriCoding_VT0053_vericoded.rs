use vstd::prelude::*;
verus! {
fn numpy_flip(m: Vec<f32>) -> (result: Vec<f32>){
    let mut result: Vec<f32> = Vec::new();
    let n = m.len();
    let mut i: usize = 0;
    while i < n
    {
        let index = n - 1 - i;
        result.push(m[index]);
        i = i + 1;
    }
    result
}
}
fn main() {}