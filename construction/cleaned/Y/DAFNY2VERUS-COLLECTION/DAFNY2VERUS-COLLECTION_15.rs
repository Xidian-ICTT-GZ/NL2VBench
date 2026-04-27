use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method: mfirstCero
// Returns the index of the first element that is zero, or v.Length if none exists
fn mfirst_cero(v: &[i32]) -> (i: usize)
    ensures
        0 <= i && i <= v.len(),
        forall|j: int| 0 <= j && j < i ==> v[j as int] != 0,
        i != v.len() ==> v[i as int] == 0,
{
    let mut i = 0;

    while i < v.len() && v[i] != 0
        invariant
            0 <= i && i <= v.len(),
            forall|j: int| 0 <= j && j < i ==> v[j as int] != 0,
        decreases v.len() - i
    {
        i = i + 1;
    }

    i
}

fn main() {}
}