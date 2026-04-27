use vstd::prelude::*;
verus! {
fn swap_first_and_last(a: &mut Vec<i32>){
    let last = a.len() - 1;
    let first_val = a[0];
    let last_val = a[last];
    a[0] = last_val;
    a[last] = first_val;
}
fn main() {}
}