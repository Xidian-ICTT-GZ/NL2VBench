use vstd::prelude::*;
verus! {
fn look_for_min(a: &[int], i: usize) -> (m: usize){
    let mut min_idx = i;
    let mut j = i + 1;
    while j < a.len()
    {
        if a[j] < a[min_idx] {
            min_idx = j;
        }
        j += 1;
    }
    min_idx
}
fn main() {
}
}