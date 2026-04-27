use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method to square each element of a vector
fn square_elements(a: Vec<int>) -> (squared: Vec<int>)
    ensures
        squared.len() == a.len(),
        forall|i: int| 0 <= i < a.len() ==> squared[i] == a[i] * a[i],
{
    let mut squared = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i as int && i as usize <= a.len(),
            squared.len() == i,
            forall|k: int| 0 <= k < i as int ==> squared[k] == a[k] * a[k],
        decreases 
            a.len() - i
    {
        squared.push(a[i] * a[i]);
        i += 1;
    }

    squared
}

// Empty main function required
fn main() {}
}