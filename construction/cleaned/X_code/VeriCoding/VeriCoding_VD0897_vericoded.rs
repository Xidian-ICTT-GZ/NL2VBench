use vstd::prelude::*;
verus! {

fn find_min_index(a: &[int], s: usize, e: usize) -> (min_i: usize)
{
    let mut min_index = s;
    let mut i = s + 1;
    while i < e
    {
        if a[i] < a[min_index] {
            min_index = i;
        }
        i += 1;
    }
    min_index
}
fn main() {}
}