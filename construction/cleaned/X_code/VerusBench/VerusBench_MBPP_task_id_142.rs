use vstd::prelude::*;

fn main() {}

verus! {

fn count_identical_position(arr1: &Vec<i32>, arr2: &Vec<i32>, arr3: &Vec<i32>) -> (count: usize){
    let mut count = 0;
    let mut index = 0;
    while index < arr1.len() {
        if arr1[index] == arr2[index] && arr2[index] == arr3[index] {
            count += 1;
        }
        index += 1;
    }
    count
}

} // verus!