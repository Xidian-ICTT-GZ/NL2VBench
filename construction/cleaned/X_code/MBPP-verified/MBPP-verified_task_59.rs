use vstd::prelude::*;
verus! {
fn find_first_odd(arr: &Vec<u32>) -> (index: Option<usize>){
    let input_len = arr.len();
    let mut index = 0;
    while index < arr.len()
    {
        if (arr[index] % 2 != 0) {
            return Some(index);
        }
        index += 1;
    }
    None
}
fn main() {}
} 