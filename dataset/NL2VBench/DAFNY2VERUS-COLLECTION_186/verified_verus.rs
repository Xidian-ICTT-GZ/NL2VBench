use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Modifies the array in place by replacing all negative elements with 0.
fn zap_negatives(a: &mut Vec<i32>)
    ensures
        forall|i: int| 0 <= i < a.len() ==> 
            #[trigger] a[i] == if old(a)[i] < 0 { 0 } else { old(a)[i] },
        a.len() == old(a).len(),
{
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|j: int| 0 <= j < i ==> 
                #[trigger] a[j] == if old(a)[j] < 0 { 0 } else { old(a)[j] },
            forall|j: int| i <= j && j < a.len() ==> a[j] == old(a)[j],
            a.len() == old(a).len(),
        decreases
        a.len() - i
    {
        if a[i] < 0 {
            a[i] = 0;
        }
        i += 1;
    }
}

} // end verus!

// Required empty main (verification happens in `verus!` block, not in executable tests)
fn main() {}