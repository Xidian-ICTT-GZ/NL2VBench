use vstd::prelude::*;
verus! {
fn where_fn(condition: Vec<bool>, x: Vec<f32>, y: Vec<f32>) -> (result: Vec<f32>){
    let mut result: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < condition.len()
    {
        let val = if condition[i] { x[i] } else { y[i] };
        result.push(val);
        i += 1;
    }
    result
}
}
fn main() {}