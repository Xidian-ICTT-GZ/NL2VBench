use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn getmini(a: &Vec<i32>) -> (mini: usize){
    let mut min: i32 = a[0];
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] < min {
            min = a[i];
        }
        i += 1;
    }
    let mut k: usize = 0;
    while k < a.len()
    {
        if a[k] == min {
            return k;
        }
        k += 1;
    }
    0
}
fn main() {}
}