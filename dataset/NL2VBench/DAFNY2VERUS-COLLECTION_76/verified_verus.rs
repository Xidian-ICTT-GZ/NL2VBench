use vstd::prelude::*;

verus! {

// Function: min
// Returns the minimum value among the first `n` elements of array `a`
fn min(a: &Vec<i32>, n: usize) -> (min: i32)
    requires
        0 < n && n <= a.len(),
    ensures
        exists|i: int| 0 <= i && i < n as int && a[i] == min,
        forall|i: int| 0 <= i && i < n as int ==> a[i] >= min,
{
    let mut min = a[0];
    let mut i: usize = 1;

    while i < n && i < a.len()
        invariant
            i <= n,
            i <= a.len(),
            exists|j: int| 0 <= j && j < i as int && a[j] == min,
            forall|j: int| 0 <= j && j < i as int ==> a[j] >= min,
        decreases 
            n - i
    {
        if a[i] < min {
            min = a[i];
        }
        i = i + 1;
    }

    min
}

fn main() {}

}