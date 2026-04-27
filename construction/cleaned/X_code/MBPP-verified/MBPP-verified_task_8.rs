use vstd::prelude::*;
verus! {
fn contains_k(arr: &Vec<i32>, k: i32) -> (result: bool){
    let mut index = 0;
    while index < arr.len()
    {
        if (arr[index] == k) {
            return true;
        }
        index += 1;
    }
    false
}
fn main() {}
} 