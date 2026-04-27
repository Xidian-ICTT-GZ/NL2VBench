use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: FindMax
// Returns the index of the maximum element in a non-empty array
fn find_max(a: &[i32]) -> (i: usize)
    requires
        a.len() > 0,
    ensures
        0 <= i && i < a.len(),
        forall|k: int| 0 <= k && k < a.len() ==> a[k] <= a[i as int],
{
    let mut i = 0;
    let mut index = 1;

    while index < a.len()
        invariant
            0 < index && index <= a.len(),
            0 <= i && i < index,
            forall|k: int| 0 <= k && k < index as int ==> a[k] <= a[i as int],
        decreases a.len()-index
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