use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Iterative method to reverse an array of characters
fn reverse(a: &[char]) -> (b: Vec<char>)
    requires
        a.len() > 0,
    ensures
        b.len() == a.len(),
        forall|k: int| 0 <= k < a.len() as int ==> b[k as int] == a[(a.len() - 1) - k as int],
{
    let mut b: Vec<char> = Vec::with_capacity(a.len());
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            b.len() == i,
            forall|k: int| 0 <= k < i as int ==> b[k as int] == a[(a.len() - 1) - k as int],
        decreases
            a.len() - i
    {
        let index_in_a = a.len() - 1 - i;
        b.push(a[index_in_a]);
        i += 1;
    }

    b
}

// Empty main function (required)
fn main() {}
}