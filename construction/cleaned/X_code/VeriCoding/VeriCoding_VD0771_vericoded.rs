use vstd::prelude::*;
verus! {
fn swap_first_and_last(a: &mut Vec<i32>){
    let n = a.len();
    let first = a[0];
    let last = a[n - 1];
    a[0] = last;
    a[n - 1] = first;
}
fn main() {
}
}