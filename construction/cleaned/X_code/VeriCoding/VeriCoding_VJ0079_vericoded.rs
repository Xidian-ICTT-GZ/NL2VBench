use vstd::prelude::*;
verus! {
fn element_wise_subtract(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < arr1.len()
    {
        let diff = arr1[i] - arr2[i];
        result.push(diff);
        i = i + 1;
    }
    result
}
}
fn main() {}