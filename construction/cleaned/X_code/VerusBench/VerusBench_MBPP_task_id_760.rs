use vstd::prelude::*;

fn main() {}

verus! {

fn has_only_one_distinct_element(arr: &Vec<i32>) -> (result: bool){
    if arr.len() <= 1 {
        return true;
    }
    let mut index = 1;
    while index < arr.len() {
        if arr[0] != arr[index] {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!