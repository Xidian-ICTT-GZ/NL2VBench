use vstd::prelude::*;

verus! {

/// Rotates the array `a` to the left by `offset` positions.
/// Returns a new array `b` such that:
///   b[i] == a[(i + offset) % a.len()]
fn rotate(a: Vec<int>, offset: usize) -> (b: Vec<int>)
    requires
        0 <= offset,
        a.len() < 1000000, // added relaxation to prevent overflow
        offset < 1000000, // added relaxation to prevent overflow
        a.len() < usize::MAX / 2, // added relaxation to prevent overflow
        offset < usize::MAX / 2, // added relaxation to prevent overflow
    ensures
        b.len() == a.len(),
        forall|i: int| 0 <= i && i < a.len() ==> b[i] == a[(i + offset as int) % a.len() as int],
{
    let mut b: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            b.len() == i,
            forall|j: int| 0 <= j && j < i as int ==> b[j] == a[(j + offset as int) % a.len() as int],
            i < usize::MAX / 2, // added relaxation to prevent overflow
            offset < usize::MAX / 2, // added relaxation to prevent overflow
            a.len() < usize::MAX / 2, // added relaxation to prevent overflow
        decreases
            a.len() - i,
    {
        let idx = (i + offset) % a.len();
        b.push(a[idx]);
        i += 1;
    }

    b
}

fn main() {}

}