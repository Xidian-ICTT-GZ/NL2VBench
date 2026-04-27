use vstd::prelude::*;
verus! {
fn is_sorted(arr: &Vec<i32>) -> (result: bool){
    let mut i = 0;
    while i < arr.len() - 1
    {
        if arr[i] > arr[i + 1] {
            return false;
        }
        i += 1;
    }
    true
}
}
fn main() {}