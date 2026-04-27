use vstd::prelude::*;
verus! {
fn copy(arr: &Vec<i8>) -> (result: Vec<i8>){
    let mut result: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < arr.len()
    {
        result.push(arr[i]);
        i = i + 1;
    }
    result
}
}
fn main() {}