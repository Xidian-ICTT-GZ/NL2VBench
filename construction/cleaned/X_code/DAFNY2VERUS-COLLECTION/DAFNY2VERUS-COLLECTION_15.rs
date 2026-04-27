use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn mfirst_cero(v: &[i32]) -> (i: usize){
    let mut i = 0;
    while i < v.len() && v[i] != 0
    {
        i = i + 1;
    }
    i
}
fn main() {}
}