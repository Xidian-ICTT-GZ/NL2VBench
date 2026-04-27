use vstd::prelude::*;
verus! {
fn nanargmax(a: Vec<i8>) -> (idx: usize){
    let mut max_idx: usize = 0;
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] > a[max_idx] {
            max_idx = i;
        }
        i = i + 1;
    }
    max_idx
}
}
fn main() {}