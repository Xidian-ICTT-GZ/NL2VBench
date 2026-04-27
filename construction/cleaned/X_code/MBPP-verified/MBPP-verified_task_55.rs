use vstd::prelude::*;
verus! {
fn find_first_occurrence(arr: &Vec<i32>, target: i32) -> (index: Option<usize>){
    let mut index = 0;
    while index < arr.len()
    {
        if arr[index] == target {
            return Some(index);
        }
        index += 1;
    }
    None
}
fn main() {}
} 