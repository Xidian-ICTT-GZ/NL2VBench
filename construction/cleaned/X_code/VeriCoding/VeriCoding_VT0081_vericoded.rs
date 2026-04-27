use vstd::prelude::*;
verus! {
fn make_outer(a: Vec<f32>, b: Vec<f32>) -> (result: Vec<Vec<f32>>){
    let mut res: Vec<Vec<f32>> = Vec::new();
    res.push(a);
    res.push(b);
    res
}
fn vstack(a: Vec<f32>, b: Vec<f32>) -> (result: Vec<Vec<f32>>){
    let result = make_outer(a, b);
    result
}
}
fn main() {}