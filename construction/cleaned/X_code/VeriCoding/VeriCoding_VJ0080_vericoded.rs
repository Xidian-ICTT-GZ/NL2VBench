use vstd::prelude::*;
verus! {
fn all_elements_equals(arr: &Vec<i32>, element: i32) -> (result: bool){
    let mut i: usize = 0;
    let mut res: bool = true;
    while i < arr.len()
    {
        res = res && (arr[i] == element);
        i += 1;
    }
    res
}
}
fn main() {}