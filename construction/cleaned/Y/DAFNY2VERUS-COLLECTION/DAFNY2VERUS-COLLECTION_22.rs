use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Checks whether `n` is greater than all elements in the array `a`
fn is_greater(n: i32, a: Vec<i32>) -> (result: bool)
    ensures
        result ==> forall|i: int| 0 <= i < a.len() ==> n > a[i],
        !result ==> exists|i: int| 0 <= i < a.len() && n <= a[i],
{
    let mut result = true;
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            result ==> forall|k: int| 0 <= k < i as int ==> n > a[k],
            !result ==> exists|k: int| 0 <= k < i as int && n <= a[k],
        decreases 
            a.len() - i
    {
        if n <= a[i] {
            result = false;
            return result;
        }
        i = i + 1;
    }

    result
}

fn main() {}

}