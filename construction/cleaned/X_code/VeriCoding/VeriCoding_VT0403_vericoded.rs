use vstd::prelude::*;
verus! {
fn chebweight(x: Vec<f32>) -> (result: Vec<f32>){
    let n: usize = x.len();
    let last: usize = n - 1;
    let mut w: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let val: f32 = if i == 0 || i == last { 0.5f32 } else { 1.0f32 };
        w.push(val);
        i = i + 1;
    }
    w
}
}
fn main() {}