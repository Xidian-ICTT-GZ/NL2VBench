use vstd::prelude::*;
verus! {
fn contains(str: &Vec<i32>, key: i32) -> (result: bool){
    let mut i = 0;
    while i < str.len()
    {
        if (str[i] == key) {
            return true;
        }
        i += 1;
    }
    false
}
fn remove_elements(arr1: &Vec<i32>, arr2: &Vec<i32>) -> (result: Vec<i32>){
    let mut output_str = Vec::new();
    let mut index: usize = 0;
    while index < arr1.len()
    {
        if (!contains(arr2, arr1[index])) {
            output_str.push(arr1[index]);
        }
        index += 1;
    }
    output_str
}
fn main() {}
} 