use vstd::prelude::*;
verus! {
fn single(x: &[i32], y: &[i32]) -> (b: Vec<i32>){
    let mut result = Vec::new();
    for i in 0..x.len()
    {
        result.push(x[i]);
    }
    for i in 0..y.len()
    {
        result.push(y[i]);
    }
    result
}
fn main() {
}
}