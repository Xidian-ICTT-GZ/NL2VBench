use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: FindSmallest
// Returns the smallest element in a non-empty array of integers
fn find_smallest(s: Vec<int>) -> (min: int)
    requires
        s.len() > 0,
    ensures
        forall|i: int| 0 <= i && i < s.len() ==> min <= s[i],
        exists|i: int| 0 <= i && i < s.len() && min == s[i],
{
    let mut min = s[0];
    let mut i = 1;

    while i < s.len()
        invariant
            0 <= i && i <= s.len(),
            forall|k: int| 0 <= k && k < i ==> min <= s[k],
            exists|k: int| 0 <= k && k < i && min == s[k],
        decreases 
            s.len() - i
    {
        if s[i] < min {
            min = s[i];
        }
        i += 1;
    }

    min
}

fn main() {}

}