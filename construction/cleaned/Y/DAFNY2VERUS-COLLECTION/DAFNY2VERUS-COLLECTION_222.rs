use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method: reverse
// Reverses the array in-place
fn reverse(a: &mut Vec<i32>)
    ensures
        forall|i: int| 0 <= i < a.len() ==> a[i] == old(a)[a.len() - 1 - i],
{
    let mut i: usize = 0;

    while i < a.len() / 2
        invariant
            0 <= i && i <= a.len() / 2,
            forall|k: int| 0 <= k < i as int || (a.len() - 1 - (i as int)) < k && k < a.len() ==> a[k] == old(a)[a.len() - 1 - k],
            forall|k: int| i as int <= k && k < a.len() - i as int ==> a[k] == old(a)[k],
        decreases a.len() / 2 - i,
    {
        // Swap a[i] and a[a.len() - 1 - i]
        let j = a.len() - 1 - i;

        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;

        i = i + 1;
    }
}

fn main() {}
}