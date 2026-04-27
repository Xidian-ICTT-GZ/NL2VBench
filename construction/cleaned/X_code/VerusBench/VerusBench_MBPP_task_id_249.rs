use vstd::prelude::*;

fn main() {}

verus! {

fn contains(arr: &Vec<i32>, key: i32) -> (result: bool){
    let mut i = 0;
    while i < arr.len() {
        if (arr[i] == key) {
            return true;
        }
        i += 1;
    }
    false
}

fn intersection(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let mut output_arr = Vec::new();
    let mut index = 0;
    while index < arr1.len() {
        if (contains(arr2, arr1[index]) && !contains(&output_arr, arr1[index])) {
            output_arr.push(arr1[index]);
        }
        index += 1;
    }
    output_arr
}

} // verus!