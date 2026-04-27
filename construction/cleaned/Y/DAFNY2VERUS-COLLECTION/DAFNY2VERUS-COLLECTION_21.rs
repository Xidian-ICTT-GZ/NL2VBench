use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Creates an array of length `n` containing values 0, 1, 2, ..., n-1
fn array_up_to_n(n: usize) -> (a: Vec<usize>)
    requires
        n >= 0,
    ensures
        a.len() == n,
        forall|j: int| 0 < j && j < n as int ==> a[j] >= 0,
        forall|j: int, k: int| 0 <= j && j <= k && k < n as int ==> a[j] <= a[k],
{
    let mut a: Vec<usize> = Vec::new();
    let mut i: usize = 0;

    while i < n
        invariant
            0 <= i && i <= n,
            a.len() == i,
            forall|k: int| 0 <= k && k < i as int ==> a[k] >= 0,
            forall|k: int| 0 <= k && k < i as int ==> a[k] == k,
            forall|j: int, k: int| 0 <= j && j <= k && k < i as int ==> a[j] <= a[k],
        decreases
            n - i
    {
        a.push(i);
        i += 1;
    }

    a
}

fn main() {}

}