use vstd::prelude::*;
verus! {
fn place(arr: Vec<f32>, mask: Vec<bool>, vals: Vec<f32>) -> (result: Vec<f32>){
    let mut result = Vec::new();
    let mut val_idx: usize = 0;
    let mut i: usize = 0;
    while i < arr.len()
    {
        if mask[i] {
            result.push(vals[val_idx]);
            val_idx = if val_idx + 1 == vals.len() { 0 } else { val_idx + 1 };
        } else {
            result.push(arr[i]);
        }
        i = i + 1;
    }
    result
}
}
fn main() {}