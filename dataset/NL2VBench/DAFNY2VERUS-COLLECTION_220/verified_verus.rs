use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Inverts an array of integers in place.
fn invert_array(a: &mut Vec<i32>)
    ensures
        forall|i: int| 0 <= i && i < a.len() ==> a[i] == old(a)[a.len() - 1 - i],
{
    let mut index = 0;

    while index < a.len() / 2
        invariant
            0 <= index && index <= a.len() / 2,
            forall|i: int| 0 <= i && i < index ==> a[i] == old(a)[a.len() - 1 - i],
            forall|i: int| 0 <= i && i < index ==> a[a.len() - 1 - i] == old(a)[i],
            forall|i: int| index <= i && i < (a.len() - index as usize) ==> a[i] == old(a)[i],
        decreases a.len() / 2 - index,
    {
        // Swap elements at `index` and `a.len() - 1 - index`
        let j = a.len() - 1 - index; 

        let temp = a[index];
        a[index] = a[j];
        a[j] = temp;

        index += 1;
    }
}

fn main() {}
}