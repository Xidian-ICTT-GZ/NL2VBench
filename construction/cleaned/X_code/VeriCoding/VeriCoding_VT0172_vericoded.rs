use vstd::prelude::*;
verus! {
fn create_row(v: &Vec<f32>, i: usize) -> (row: Vec<f32>){
    let mut row: Vec<f32> = Vec::with_capacity(v.len());
    let mut j: usize = 0;
    while j < v.len()
    {
        if j == i {
            row.push(v[i]);
        } else {
            row.push(0.0f32);
        }
        j = j + 1;
    }
    row
}
fn diagflat(v: Vec<f32>) -> (result: Vec<Vec<f32>>){
    let mut result: Vec<Vec<f32>> = Vec::with_capacity(v.len());
    let mut i: usize = 0;
    while i < v.len()
    {
        let row = create_row(&v, i);
        result.push(row);
        i = i + 1;
    }
    result
}
}
fn main() {}