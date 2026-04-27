use vstd::prelude::*;
verus! {
fn numpy_full_like(a: Vec<f32>, fill_value: f32) -> (result: Vec<f32>){
    let n = a.len();
    let mut r: Vec<f32> = Vec::new();
    let mut i: usize = 0usize;
    while i < n
    {
        let old_len = r.len();
        r.push(fill_value);
        i += 1;
    }
    r
}
}
fn main() {}