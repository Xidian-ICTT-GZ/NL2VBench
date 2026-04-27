use vstd::prelude::*;
verus! {
fn element_wise_divide(arr1: &Vec<u32>, arr2: &Vec<u32>) -> (result: Vec<u32>){
    let mut result: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    while i < arr1.len()
    {
        let val = arr1[i] / arr2[i];
        result.push(val);
        i = i + 1;
    }
    result
}
}
fn main() {}