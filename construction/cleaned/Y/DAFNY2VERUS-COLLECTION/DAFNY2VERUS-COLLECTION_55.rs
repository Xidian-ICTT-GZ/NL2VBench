use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: find
// Searches for the first occurrence of `key` in the array `a`
// Returns the index of the key, or a.Length if not found
fn find(a: Vec<i32>, key: i32) -> (index: usize)
    requires
        a.len() > 0,
    ensures
        0 <= index && index <= a.len(),
        index < a.len() ==> a[index as int] == key,
{
    let mut index: usize = 0;

    while index < a.len() && a[index] != key
        invariant
            0 <= index && index <= a.len(),
            forall|x: int| 0 <= x && x < index as int ==> a[x] != key,
        decreases a.len() - index
    {
        index = index + 1;
    }

    index
}

fn main() {}
}