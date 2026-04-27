use vstd::prelude::*;
verus! {
fn take(arr: Vec<f32>, indices: Vec<usize>) -> (result: Vec<f32>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < indices.len()
    {
        let index = indices[i];
        let value = arr[index];
        result.push(value);
        i = i + 1;
    }
    result
}
}
fn main() {}