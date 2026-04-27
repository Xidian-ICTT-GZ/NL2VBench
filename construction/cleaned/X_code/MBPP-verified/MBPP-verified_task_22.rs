use vstd::prelude::*;
verus! {
fn max_difference(arr: &Vec<i32>) -> (diff: i32){
    let mut min_val = arr[0];
    let mut max_val = arr[0];
    let mut index = 1;
    while index < arr.len()
    {
        if (arr[index] < min_val) {
            min_val = arr[index];
        } else if (arr[index] > max_val) {
            max_val = arr[index];
        }
        index += 1;
    }
    max_val - min_val
}
fn main() {}
} 