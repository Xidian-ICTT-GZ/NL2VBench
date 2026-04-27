use vstd::prelude::*;
verus! {
fn find_max(a: &[i32]) -> (i: usize)
{
    let mut max_idx = 0;
    let mut j = 1;
    while j < a.len()
    {
        if a[j] > a[max_idx] {
            max_idx = j;
        }
        j += 1;
    }
    max_idx
}
fn main() {}
}