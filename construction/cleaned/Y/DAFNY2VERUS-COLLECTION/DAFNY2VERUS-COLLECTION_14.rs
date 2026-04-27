use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: max
// Returns the maximum value in a non-empty array of integers.
// If the array is empty, returns 0.
fn max(a: &Vec<i32>) -> (max: i32)
    requires
        a.len() > 0 ==> true, // In Verus, Vec is always valid; null check is implicit
    ensures
        forall|j: int| 0 <= j && j < a.len() ==> max >= a[j],
        a.len() > 0 ==> exists|j: int| 0 <= j && j < a.len() && max == a[j],
{
    if a.len() == 0 {
        return 0;
    }

    let mut max = a[0];
    let mut i = 1;

    while i < a.len()
        invariant
            i <= a.len(),
            forall|j: int| 0 <= j && j < i ==> max >= a[j],
            exists|j: int| 0 <= j && j < i && max == a[j],
        decreases
            a.len()-i
    {
        if a[i] > max {
            max = a[i];
        }
        i = i + 1;
    }

    max
}

} // end verus!

fn main() {}