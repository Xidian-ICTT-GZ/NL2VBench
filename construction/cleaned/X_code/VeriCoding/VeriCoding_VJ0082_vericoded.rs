use vstd::prelude::*;
verus! {
fn bitxor_i32(a: i32, b: i32) -> i32 { a ^ b }
fn bit_wise_xor(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let n = arr1.len();
    let mut result: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let x = arr1[i] ^ arr2[i];
        result.push(x);
        i = i + 1;
    }
    result
}
}
fn main() {}