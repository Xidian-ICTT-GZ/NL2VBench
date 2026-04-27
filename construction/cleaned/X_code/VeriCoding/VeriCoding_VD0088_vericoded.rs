use vstd::prelude::*;
verus! {
fn swap(arr: &mut Vec<i32>, i: usize, j: usize){
    let tmp_i = arr[i];
    let tmp_j = arr[j];
    arr[i] = tmp_j;
    arr[j] = tmp_i;
}
fn main() {}
}