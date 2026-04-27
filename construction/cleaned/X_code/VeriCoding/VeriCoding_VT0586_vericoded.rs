use vstd::prelude::*;
verus! {
fn max(a: Vec<i8>) -> (result: i8){
    let mut best_idx: usize = 0;
    let mut best_val: i8 = a[0];
    let mut i: usize = 1;
    while i < a.len()
    {
        let ai = a[i];
        if ai > best_val {
            best_val = ai;
            best_idx = i;
        }
        i += 1;
    }
    best_val
}
}
fn main() {}