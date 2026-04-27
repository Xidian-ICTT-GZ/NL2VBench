use vstd::prelude::*;

verus! {

/// Determines whether there exists a common element between arrays `r` and `x`.
/// Assumes `r` and `x` are non-negative and `x` is sorted in ascending order.
fn tangent(r: Vec<u64>, x: Vec<u64>) -> (b: bool)
    requires
        // x is sorted in ascending order
        forall|i: int, j: int| 0 <= i <= j < x.len() ==> x[i] <= x[j],
        // All elements in r and x are non-negative (redundant for u64, but kept for spec)
        forall|i: int, j: int| (0 <= i < r.len() && 0 <= j < x.len()) ==> (r[i] >= 0 && x[j] >= 0),
    ensures
        !b ==> forall|i: int, j: int| 0 <= i < r.len() && 0 <= j < x.len() ==> r[i] != x[j],
        b ==> exists|i: int, j: int| 0 <= i < r.len() && 0 <= j < x.len() && r[i] == x[j],
{
    let mut temp_b = false;
    let mut k: usize = 0;

    while k < r.len() && !temp_b
        invariant
            0 <= k && k <= r.len(),
            temp_b ==> exists|i: int, j: int| 0 <= i < r.len() && 0 <= j < x.len() && r[i] == x[j],
            !temp_b ==> forall|i: int, j: int| 0 <= i < k && 0 <= j < x.len() ==> r[i] != x[j],
        decreases r.len() - k,
    {
        let mut l: usize = 0;
        let mut tangent_missing = false;

        while l < x.len() && !tangent_missing && k < r.len()
            invariant
                0 <= l && l <= x.len(),
                temp_b ==> exists|i: int, j: int| 0 <= i < r.len() && 0 <= j < x.len() && r[i] == x[j],
                !temp_b ==> forall|i: int| 0 <= i < l ==> r[k as int] != x[i],
                tangent_missing ==> l == x.len(),
            decreases x.len() - l
        {
            assert(l < x.len());
            if r[k] == x[l] {
                temp_b = true;
            }

            if l == x.len() - 1 && r[k] < x[l] {
                tangent_missing = true;
            }

            l = l + 1;
        }

        k = k + 1;
    }

    temp_b
}

fn main() {}
}