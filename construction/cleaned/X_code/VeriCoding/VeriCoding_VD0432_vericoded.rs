use vstd::prelude::*;
verus! {
fn find_min(a: &[int], lo: usize) -> (minIdx: usize){
    let mut min_idx = lo;
    let mut i = lo + 1;
    while i < a.len()
    {
        if a[i] < a[min_idx] {
            min_idx = i;
        }
        i += 1;
    }
    min_idx
}
fn main() {}
}