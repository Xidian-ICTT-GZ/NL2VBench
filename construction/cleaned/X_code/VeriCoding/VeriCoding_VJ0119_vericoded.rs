use vstd::prelude::*;
verus! {
fn find_first_occurrence(arr: &Vec<i32>, target: i32) -> (index: Option<usize>){
    let mut i: usize = 0;
    while i < arr.len()
    {
        if arr[i] == target {
            return Some(i);
        } else {
            i = i + 1;
        }
    }
    None
}
}
fn main() {}