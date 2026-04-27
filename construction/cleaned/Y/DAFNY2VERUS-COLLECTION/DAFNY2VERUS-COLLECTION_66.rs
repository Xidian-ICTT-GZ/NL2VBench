use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;
use vstd::set::*;

verus! {

/// Returns the minimum element of the array.
fn min_array(a: Vec<int>) -> (r: int)
    requires
        a.len() > 0,
    ensures
        forall|i: int| 0 <= i < a.len() ==> r <= a[i],
        exists|i: int| 0 <= i < a.len() && r == a[i],
{
    let mut r = a[0];
    let mut i = 1;
    while i < a.len() 
        invariant
            0 <= i && i <= a.len(),
            forall|x: int| 0 <= x < i ==> r <= a[x],
            exists|x: int| 0 <= x < i && r == a[x],
        decreases
            a.len() - i
    {
        if r > a[i] {
            r = a[i];
        }
        i = i + 1;
    }
    r
}

fn main() {}
}