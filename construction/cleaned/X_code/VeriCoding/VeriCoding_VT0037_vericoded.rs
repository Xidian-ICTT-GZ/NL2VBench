use vstd::prelude::*;
verus! {
fn numpy_append(arr: Vec<f32>, values: Vec<f32>) -> (result: Vec<f32>){
    let mut result = arr;
    let mut i = 0;
    while i < values.len()
    {
        result.push(values[i]);
        i += 1;
    }
    result
}
}
fn main() {}