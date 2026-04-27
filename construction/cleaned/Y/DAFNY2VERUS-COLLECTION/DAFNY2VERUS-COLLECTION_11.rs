use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Finds the index of the maximum element in a non-empty array.
fn find_max(a: &[i32]) -> (max_idx: usize)
    requires
        a.len() > 0,
    ensures
        0 <= max_idx && max_idx < a.len(),
        forall|j: int| 0 <= j < a.len() ==> a[max_idx as int] >= a[j],
{
    let mut max_idx: usize = 0;
    let mut i: usize = 1;

    while i < a.len()
        invariant
            1 <= i && i <= a.len(),
            0 <= max_idx && max_idx < i,
            forall|j: int| 0 <= j < i as int ==> a[max_idx as int] >= a[j],
        decreases a.len() - i
    {
        if a[i] > a[max_idx] {
            max_idx = i;
        }
        i += 1;
    }

    max_idx
}

} // end verus!

fn main() {}