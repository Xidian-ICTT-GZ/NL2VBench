use vstd::prelude::*;
verus! {
fn longest_prefix(a: &[i32], b: &[i32]) -> (i: usize){
    let mut i: usize = 0;
    while i < a.len() && i < b.len() && a[i] == b[i]
    {
        i = i + 1;
    }
    i
}
fn main() {
}
}