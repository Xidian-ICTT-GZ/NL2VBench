use vstd::prelude::*;
verus! {
fn is_odd_at_odd_index(arr: &Vec<usize>) -> (result: bool){
    let mut index = 0;
    while index < arr.len()
    {
        if (index % 2) != (arr[index] % 2) {
            return false;
        }
        index += 1;
    }
    true
}
}
fn main() {}