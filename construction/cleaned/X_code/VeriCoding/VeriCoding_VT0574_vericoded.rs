use vstd::prelude::*;
verus! {
fn numpy_where(condition: Vec<bool>, x: Vec<f32>, y: Vec<f32>) -> (result: Vec<f32>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < condition.len()
    {
        if condition[i] {
            result.push(x[i]);
        } else {
            result.push(y[i]);
        }
        i = i + 1;
    }
    result
}
}
fn main() {}