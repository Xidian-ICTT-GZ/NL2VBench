use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn find_max(a: &[i32]) -> (i: usize){
    let mut i = 0;
    let mut index = 1;
    while index < a.len()
    {
        if a[index] > a[i] {
            i = index;
        }
        index = index + 1;
    }
    i
}
fn main() {}
}