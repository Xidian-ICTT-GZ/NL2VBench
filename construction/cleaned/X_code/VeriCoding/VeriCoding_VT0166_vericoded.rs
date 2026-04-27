use vstd::prelude::*;
verus! {
fn argmin(a: Vec<i8>) -> (result: usize){
    let mut min_idx: usize = 0;
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] < a[min_idx] {
            min_idx = i;
        }
        i += 1;
    }
    min_idx
}
}
fn main() {}