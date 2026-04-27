use vstd::prelude::*;
verus! {
fn contains_k(arr: &Vec<i32>, k: i32) -> (result: bool){
    let n: usize = arr.len();
    let mut i: usize = 0;
    let mut found: bool = false;
    while i < n
    {
        let ai = arr[i];
        let new_found = found || (ai == k);
        found = new_found;
        i = i + 1;
    }
    found
}
}
fn main() {}