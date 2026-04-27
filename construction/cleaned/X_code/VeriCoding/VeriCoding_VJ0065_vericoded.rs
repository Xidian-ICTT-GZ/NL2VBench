use vstd::prelude::*;
verus! {
fn count_true(arr: &Vec<bool>) -> (count: u64){
    let mut i: usize = 0;
    let mut c: u64 = 0u64;
    while i < arr.len()
    {
        let b = arr[i];
        if b {
            c = c + 1;
        }
        i = i + 1;
    }
    c
}
}
fn main() {}