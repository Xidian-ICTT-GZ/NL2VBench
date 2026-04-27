use vstd::prelude::*;
verus! {
fn find_odd_numbers(arr: &Vec<u32>) -> (odd_numbers: Vec<u32>){
    let mut odd_numbers: Vec<u32> = Vec::new();
    let input_len = arr.len();
    let mut index = 0;
    while index < arr.len()
    {
        if (arr[index] % 2 != 0) {
            odd_numbers.push(arr[index]);
        }
        index += 1;
    }
    odd_numbers
}
fn main() {}
} 