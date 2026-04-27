use vstd::prelude::*;
verus! {
fn mlast_maximum(v: &[i32]) -> (i: usize){
    let mut max_idx = 0;
    let mut i = 1;
    while i < v.len()
    {
        if v[i] >= v[max_idx] {
            max_idx = i;
        }
        i += 1;
    }
    max_idx
}
fn main() {}
}