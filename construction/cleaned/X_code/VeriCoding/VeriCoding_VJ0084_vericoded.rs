use vstd::prelude::*;
verus! {
fn contains_value(arr2: &Vec<i32>, val: i32) -> (result: bool){
    let mut j = 0;
    while j < arr2.len()
    {
        if arr2[j] == val {
            return true;
        }
        j += 1;
    }
    false
}
fn any_value_exists(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: bool){
    let mut i = 0;
    while i < arr1.len()
    {
        if contains_value(arr2, arr1[i]) {
            return true;
        }
        i += 1;
    }
    false
}
}
fn main() {}