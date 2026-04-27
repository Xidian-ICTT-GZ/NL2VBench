use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Computes the minimum element in a non-empty array.
fn minimum(a: Vec<int>) -> (m: int)
    requires
        a.len() > 0,
    ensures
        exists|i: int| 0 <= i && i < a.len() && m == a[i],
        forall|i: int| 0 <= i && i < a.len() ==> m <= a[i],
{
    let mut n: usize = 0;
    let mut m: int = a[0];

    while n != a.len()
        invariant
            0 <= n && n <= a.len(),
            exists|i: int| 0 <= i && i < a.len() && m == a[i],
            forall|i: int| 0 <= i && i < n ==> m <= a[i],
        decreases   
            a.len() - n
    {
        if a[n] < m {
            m = a[n];
        }
        n = n + 1;
    }
    m
}

} // verus!

fn main() {}