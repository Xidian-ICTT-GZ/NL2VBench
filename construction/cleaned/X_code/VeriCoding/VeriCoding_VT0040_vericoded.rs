use vstd::prelude::*;
verus! {
fn atleast_2d_helper(arr: Vec<f32>) -> (result: Vec<Vec<f32>>){
    let mut new_vec_inner: Vec<f32> = Vec::new();
    let mut i = 0;
    while i < arr.len()
    {
        new_vec_inner.push(arr[i]);
        i = i + 1;
    }
    let mut result_outer: Vec<Vec<f32>> = Vec::new();
    result_outer.push(new_vec_inner);
    result_outer
}
fn atleast_2d(arr: Vec<f32>) -> (result: Vec<Vec<f32>>){
    atleast_2d_helper(arr)
}
}
fn main() {}