use vstd::prelude::*;
verus! {
fn is_sorted(a: &[i32]) -> (sorted: bool){
    let mut i: usize = 0;
    while i < a.len() - 1
    {
        if a[i] > a[i+1] {
            return false;
        }
        i = i + 1;
    }
    true
}
}
fn main() {}