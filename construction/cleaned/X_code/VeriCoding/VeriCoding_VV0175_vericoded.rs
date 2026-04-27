use vstd::prelude::*;
verus! {
fn swap(arr: &Vec<i32>, i: usize, j: usize) -> (result: Vec<i32>){
    let mut r = arr.clone();
    let ai = arr[i];
    let aj = arr[j];
    r.set(i, aj);
    r.set(j, ai);
    r
}
}
fn main() {}