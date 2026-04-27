use vstd::prelude::*;

verus! {

// Function: max
// Returns the index of a maximal element in the array using the two-pointer algorithm
fn max(a: &Vec<i32>) -> (x: usize)
    requires
        a.len() != 0,
    ensures
        0 <= x && x < a.len(),
        forall|i: int| 0 <= i && i < a.len() ==> a[i] <= a[x as int],
{
    let mut x: usize = 0;
    let mut y: usize = a.len() - 1;
    let mut m: usize = y; // Ghost variable tracking the current candidate for max index

    while x != y
            invariant
                0 <= x && x <= y && y < a.len(),
                m == x || m == y,
                forall|i: int| 0 <= i && i < x ==> a[i] <= a[m as int],
                forall|i: int| y < i && i < a.len() ==> a[i] <= a[m as int],
         decreases 
            y - x
    {
        if a[x] <= a[y] {
            x = x + 1;
            m = y;
        } else {
            y = y - 1;
            m = x;
        }
    }

    x
}

fn main() {}
}