use vstd::prelude::*;
verus! {
fn min_i8(a: i8, b: i8) -> (result: i8){
    if a <= b { a } else { b }
}
fn min(a: Vec<i8>) -> (result: i8){
    let mut i: usize = 1usize;
    let mut m: i8 = a[0];
    while i < a.len()
    {
        let old_m = m;
        let x: i8 = a[i];
        if x < m {
            m = x;
        }
        i += 1;
    }
    m
}
}
fn main() {}