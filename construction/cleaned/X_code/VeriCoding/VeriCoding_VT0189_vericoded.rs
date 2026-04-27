use vstd::prelude::*;
verus! {
fn putmask(a: Vec<f32>, mask: Vec<bool>, values: Vec<f32>) -> (result: Vec<f32>){
    let mut result_vec = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        if mask[i] {
            let index = i % values.len();
            result_vec.push(values[index]);
        } else {
            result_vec.push(a[i]);
        }
        i = i + 1;
    }
    result_vec
}
}
fn main() {}