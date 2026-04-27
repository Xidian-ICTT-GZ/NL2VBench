use vstd::prelude::*;
verus! {
fn argmax(a: Vec<i8>) -> (result: usize){
    let mut argmax_idx: usize = 0;
    let mut max_val = a[0];
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] > max_val {
            max_val = a[i];
            argmax_idx = i;
        }
        i = i + 1;
    }
    argmax_idx
}
}
fn main() {}