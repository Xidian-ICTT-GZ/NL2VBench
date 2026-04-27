use vstd::prelude::*;

fn main() {}

verus! {

fn insert_before_each(arr: &Vec<i32>, elem: i32) -> (result: Vec<i32>){
    let mut result: Vec<i32> = Vec::new();
    let mut index = 0;

    while index < arr.len() {
        result.push(elem);
        result.push(arr[index]);
        index += 1;
    }
    result
}

} // verus!