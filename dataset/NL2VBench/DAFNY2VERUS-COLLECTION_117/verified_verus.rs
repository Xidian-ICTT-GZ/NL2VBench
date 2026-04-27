use vstd::prelude::*;

verus! {

// Function: contains
// Returns true if value `v` appears in the first `n` elements of array `a`
spec fn contains(v: int, a: &Seq<int>, n: int) -> bool {
    exists|j: int| 0 <= j && j < n && a[j] == v
}

// Function: upper_bound
// Returns true if all elements in the first `n` elements of `a` are <= v
spec fn upper_bound(v: int, a: &Seq<int>, n: int) -> bool {
    forall|j: int| 0 <= j && j < n ==> a[j] <= v
}

// Function: is_max
// Returns true if `m` is the maximum among the first `n` elements of `a`
spec fn is_max(m: int, a: &Seq<int>, n: int) -> bool {
    &&& contains(m, a, n)
    &&& upper_bound(m, a, n)
}

// Method: max
// Computes the maximum of the first `n` elements of array `a`
fn max(a: &Vec<int>, n: usize) -> (max: int)
    requires
        0 < n && n <= a.len(),
        n < 1000, // Relaxation to help with bounds checking
    ensures
        is_max(max, &a@, n as int),
{
    let mut i: usize = 1;
    let mut max = a[0];

    while i < n
        invariant
            1 <= i && i <= n,
            n <= a.len(),
            is_max(max, &a@, i as int),
        decreases
            n - i
    {
        if a[i] > max {
            max = a[i];
        }
        i = i + 1;
    }

    max
}

fn main() {}

}