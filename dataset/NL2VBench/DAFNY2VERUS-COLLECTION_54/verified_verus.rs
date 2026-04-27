use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: Max
// Returns the maximum value in a natural number array, or -1 if the array is empty.
fn max(a: Vec<u64>) -> (m: i64)
    ensures
        a.len() > 0 ==> forall|k: int| 0 <= k && k < a.len() ==> m >= a[k] as i64,
        a.len() == 0 ==> m == -1,
        a.len() > 0 ==> m as u64 == a[a.len() - 1] || exists|i: int| 0 <= i && i < a.len() && m == a[i] as i64,
{
    if a.len() == 0 {
        return -1;
    }

    let mut i: usize = 0;
    let mut m: i64 = a[0] as i64;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|k: int| 0 <= k && k < i as int ==> m >= a[k] as i64,
            exists|k: int| 0 <= k && k < a.len() && m == a[k] as i64,
        decreases
            a.len() - i
    {
        if (a[i] as i64) >= m {
            m = a[i] as i64;
        }
        i += 1;
    }

    m
}

// Checker method translated to Verus (as verification-only assertions)
fn checker() {
    let a = vec![1, 2, 3, 50, 5, 51];
    let n = max(a);
    assert(n == 51); // This will be verified statically if possible
}

fn main() {}
}