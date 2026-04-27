use vstd::prelude::*;

fn main() {}

verus! {

fn element_wise_divide(arr1: &Vec<u32>, arr2: &Vec<u32>) -> (result: Vec<u32>){
    let mut output_arr = Vec::with_capacity(arr1.len());
    let mut index = 0;
    while index < arr1.len() {
        output_arr.push((arr1[index] / arr2[index]));
        index += 1;
    }
    output_arr
}

} // verus!