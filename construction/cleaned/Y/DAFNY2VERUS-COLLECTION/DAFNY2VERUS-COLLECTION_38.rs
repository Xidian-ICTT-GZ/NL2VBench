use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Returns the maximum element in the array
fn max_array(a: Vec<int>) -> (m: int)
    requires
        a.len() >= 1,
    ensures
        forall|k: int| 0 <= k < a.len() ==> m >= a[k],
        exists|k: int| 0 <= k < a.len() && m == a[k],
{
    let mut m = a[0];
    let mut index = 1;

    while index < a.len()
        invariant
            0 <= index && index <= a.len(),
            forall|k: int| 0 <= k && k < index as int ==> m >= a[k],
            exists|k: int| 0 <= k && k < index as int && m == a[k],
        decreases a.len() - index
    {
        m = if m > a[index] { m } else { a[index] };
        index = index + 1;
    }

    m
}

fn main() {}
}