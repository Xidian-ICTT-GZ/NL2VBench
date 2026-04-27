use vstd::prelude::*;
verus! {
fn remove_odd_numbers(arr: &[i32]) -> (even_list: Vec<i32>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < arr.len()
    {
        if arr[i] % 2 == 0 {
            result.push(arr[i]);
        }
        i = i + 1;
    }
    result
}
fn main() {
}
}