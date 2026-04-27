use vstd::prelude::*;
verus! {
fn amax(a: Vec<i8>) -> (result: i8){
    let n: usize = a.len();
    let mut best_idx: usize = 0usize;
    let mut best: i8 = a[0];
    let mut i: usize = 1usize;
    while i < n
    {
        let v: i8 = a[i];
        if v > best {
            best = v;
            best_idx = i;
        }
        i = i + 1;
    }
    best
}
}
fn main() {}