use vstd::prelude::*;
verus! {
fn bmat(top_left: Vec<f32>, top_right: Vec<f32>, bottom_left: Vec<f32>, bottom_right: Vec<f32>) -> (result: Vec<f32>){
    let n: usize = top_left.len();
    let mut result: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let val = top_left[i];
        result.push(val);
        i += 1;
    }
    let mut j: usize = 0;
    while j < n
    {
        let val = top_right[j];
        result.push(val);
        j += 1;
    }
    let mut k: usize = 0;
    while k < n
    {
        let val = bottom_left[k];
        result.push(val);
        k += 1;
    }
    let mut m: usize = 0;
    while m < n
    {
        let val = bottom_right[m];
        result.push(val);
        m += 1;
    }
    result
}
}
fn main() {}