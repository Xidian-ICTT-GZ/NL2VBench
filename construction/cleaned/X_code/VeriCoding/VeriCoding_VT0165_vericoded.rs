use vstd::prelude::*;
verus! {
fn argmax(arr: &Vec<i8>) -> (result: usize){
    let mut max_idx: usize = 0;
    let mut i: usize = 1;
    while i < arr.len()
    {
        if arr[i] > arr[max_idx] {
            max_idx = i;
        }
        i = i + 1;
    }
    max_idx
}
}
fn main() {}