use vstd::prelude::*;
verus! {
fn min_second_value_first(arr: &Vec<Vec<i32>>) -> (first_of_min_second: i32){
    let mut min_second_index = 0;
    let mut index = 0;
    while index < arr.len()
    {
        if arr[index][1] < arr[min_second_index][1] {
            min_second_index = index;
        }
        index += 1;
    }
    arr[min_second_index][0]
}
fn main() {}
} 