use vstd::prelude::*;
verus! {
fn bit_wise_xor(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let mut output_arr = Vec::with_capacity(arr1.len());
    let mut index = 0;
    while index < arr1.len()
    {
        output_arr.push((arr1[index] ^ arr2[index]));
        index += 1;
    }
    output_arr
}
fn main() {}
} 