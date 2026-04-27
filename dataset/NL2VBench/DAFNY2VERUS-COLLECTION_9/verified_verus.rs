use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Linear search for element `e` in array `a`
/// Returns the index of the first occurrence of `e`, or `a.len()` if not found
fn linear_search(a: &Vec<i32>, e: i32) -> (n: usize)
    ensures
        0 <= n && n <= a.len(),
        n == a.len() || a[n as int] == e,
        forall|i: int| 0 <= i && i < n as int ==> e != a[i],
{
    let mut n = 0;

    while n != a.len()
        invariant
            0 <= n && n <= a.len(),
            forall|i: int| 0 <= i && i < n as int ==> e != a[i],
         decreases 
            a.len() - n 
    {
        if e == a[n] {
            return n; // Found element: return early
        }
        n = n + 1;
    }

    n // Return a.len() if not found
}

fn main() {}

}