use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;
verus! {
fn find_max(a: &[i32]) -> (i: usize){
    let mut j = 0;
    let mut max = a[0];
    let mut index = 1;
    while index < a.len()
    {
        if max < a[index] {
            max = a[index];
            j = index;
        }
        index = index + 1;
    }
    j
}
fn main() {}
}