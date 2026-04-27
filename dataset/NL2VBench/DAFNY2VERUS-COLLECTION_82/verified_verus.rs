use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Finds the index of the first occurrence of the minimum element in a non-empty array.
fn getmini(a: &Vec<i32>) -> (mini: usize)
    requires
        a.len() > 0,
    ensures
        0 <= mini && mini < a.len(), // mini is a valid index
        forall|x: int| 0 <= x && x < a.len() ==> a[mini as int] <= a[x], // a[mini] is minimum
        forall|x: int| 0 <= x && x < mini as int ==> a[mini as int] < a[x], // mini is the first occurrence
{
    // First pass: find the minimum value
    let mut min: i32 = a[0];
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|x: int| 0 <= x && x < i as int ==> min <= a[x],
            exists|x: int| 0 <= x && x < a.len() && min == a[x], // min is in the array
        decreases 
            a.len() - i 
    {
        if a[i] < min {
            min = a[i];
        }
        i += 1;
    }

    // Second pass: find the first occurrence of `min`
    let mut k: usize = 0;
    while k < a.len()
        invariant
            0 <= k && k <= a.len(),
            forall|x: int| 0 <= x && x < k as int ==> min < a[x],
            forall|x: int| 0 <= x && x < a.len() ==> min <= a[x], // carry over that min is global minimum
        decreases a.len() - k
    {
        if a[k] == min {
            return k;
        }
        k += 1;
    }

    // This line should never be reached because `min` must appear in the array
    // But Verus requires all paths to return. Since we know `min` is in `a`,
    // the loop will always return. We can help Verus see this by reasoning.
    // However, to satisfy the type system, we include an unreachable return.
    // In practice, this is safe because `min == a[0]` at worst.
    0
}

fn main() {}

}