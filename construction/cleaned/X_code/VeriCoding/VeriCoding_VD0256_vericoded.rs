use vstd::prelude::*;
verus! {
fn max(a: &[i32], b: &[i32], i: usize, j: usize) -> (m: i32){
    let ai = a[i];
    let bj = b[j];
    if ai > bj {
        ai
    } else {
        bj
    }
}
fn main() {}
}