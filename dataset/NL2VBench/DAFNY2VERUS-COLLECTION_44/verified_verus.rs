use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;

verus! {

// We define a function that finds the *index* of the maximum element
fn find_max(a: &[i32]) -> (i: usize)
    requires
        a.len() > 0,
    ensures
        0 <= i && i < a.len(),
        forall|k: int| #![auto] 0 <= k < a.len() ==> a[k as int] <= a[i as int],
{
    let mut j = 0;
    let mut max = a[0];
    let mut index = 1;

    while index < a.len()
        invariant
            1 <= index && index <= a.len(),
            forall|k: int| #![auto] 0 <= k && k < index as int ==> max >= a[k as int],
            0 <= j && j < a.len(),
            a[j as int] == max,
        decreases a.len() - index
    {
        if max < a[index] {
            max = a[index];
            j = index;
        }
        index = index + 1;
    }

    j
}

fn main() {}
}