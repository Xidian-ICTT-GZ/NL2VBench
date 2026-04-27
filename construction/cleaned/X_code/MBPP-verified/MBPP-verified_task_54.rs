use vstd::prelude::*;
verus! {
fn count_frequency(arr: &Vec<i32>, key: i32) -> (frequency: usize){
    let mut index = 0;
    let mut counter = 0;
    while index < arr.len()
    {
        if (arr[index] == key) {
            counter += 1;
        }
        index += 1;
    }
    counter
}
fn remove_duplicates(arr: &Vec<i32>) -> (unique_arr: Vec<i32>){
    let mut unique_arr: Vec<i32> = Vec::new();
    let input_len = arr.len();
    let mut index = 0;
    while index < arr.len()
    {
        if count_frequency(&arr, arr[index]) == 1 {
            unique_arr.push(arr[index]);
        }
        index += 1;
    }
    unique_arr
}
fn main() {}
} 