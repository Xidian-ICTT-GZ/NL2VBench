use vstd::prelude::*;
verus! {
fn is_sorted(arr: &Vec<i32>) -> (is_sorted: bool){
    let mut index = 0;
    while index < arr.len() - 1
    {
        if arr[index] > arr[index + 1] {
            return false;
        }
        index += 1;
    }
    true
}
fn main() {}
} 