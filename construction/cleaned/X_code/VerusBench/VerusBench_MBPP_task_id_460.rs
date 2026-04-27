use vstd::prelude::*;

fn main() {}

verus! {

fn get_first_elements(arr: &Vec<Vec<i32>>) -> (result: Vec<i32>){
    let mut first_elem_arr: Vec<i32> = Vec::new();
    let mut index = 0;
    while index < arr.len() {
        let seq = &arr[index];
        first_elem_arr.push(seq[0]);
        index += 1;
    }
    first_elem_arr
}

} // verus!