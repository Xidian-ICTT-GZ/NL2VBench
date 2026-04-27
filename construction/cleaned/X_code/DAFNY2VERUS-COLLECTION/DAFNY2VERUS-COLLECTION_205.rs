use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn replace(arr: &mut Vec<i32>, k: i32){
    let mut i: usize = 0;
    while i < arr.len()
    {
        if arr[i] > k {
            arr[i] = -1;
        }
        i += 1;
    }
}
fn main() {}
}