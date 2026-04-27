use vstd::prelude::*;
verus! {
fn findMax(a: &[i32], n: usize) -> (r: usize){
    let mut max_idx = 0;
    let mut i = 1;
    while i < n
    {
        if a[i] > a[max_idx] {
            max_idx = i;
        }
        i += 1;
    }
    max_idx
}
fn main() {}
}