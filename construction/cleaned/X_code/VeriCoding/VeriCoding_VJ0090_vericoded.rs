use vstd::prelude::*;
verus! {
fn element_wise_multiplication(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < arr1.len()
    {
        let product = arr1[i] * arr2[i];
        result.push(product);
        i = i + 1;
    }
    result
}
}
fn main() {}