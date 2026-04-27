use vstd::prelude::*;

fn main() {}

verus! {

fn contains_consecutive_numbers(arr: &Vec<i32>) -> (is_consecutive: bool){
    let mut index = 0;
    while (index < arr.len() - 1) {
        if (arr[index] + 1 != arr[index + 1]) {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!