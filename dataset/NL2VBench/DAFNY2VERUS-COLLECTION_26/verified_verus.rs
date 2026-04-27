use vstd::prelude::*;

verus! {

// Function: array_sum
// Takes two arrays of integers of equal length and returns a new array where each element
// is the sum of the corresponding elements from `a` and `b`.
fn array_sum(a: Vec<int>, b: Vec<int>) -> (c: Vec<int>)
    requires
        a.len() == b.len(),
    ensures
        c.len() == a.len(),
        forall|i: int| 0 <= i && i < a.len() ==> a[i] + b[i] == c[i],
{
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            c.len() == i,
            forall|j: int| 0 <= j && j < i as int ==> a[j] + b[j] == c[j],
            b.len() == a.len(),
        decreases 
            a.len() - i
    {
        c.push(a[i] + b[i]);
        i += 1;
    }

    c
}

fn main() {}

}