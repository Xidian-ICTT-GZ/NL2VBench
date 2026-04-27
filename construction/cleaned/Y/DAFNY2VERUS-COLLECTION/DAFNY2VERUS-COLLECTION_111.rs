use vstd::prelude::*;

verus! {

/// Determines whether two arrays have at least one common element.
fn has_common_element(a: Vec<i32>, b: Vec<i32>) -> (result: bool)
    ensures
        result ==> exists|i: int, j: int| 0 <= i < a.len() && 0 <= j < b.len() && a[i] == b[j],
        !result ==> forall|i: int, j: int| 0 <= i < a.len() && 0 <= j < b.len() ==> a[i] != b[j],
{
    let mut i: usize = 0;
    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|k: int, j: int| 0 <= k && k < i as int && 0 <= j && j < b.len() ==> a[k] != b[j],
        decreases
            a.len() - i
    {
        let mut j: usize = 0;
        while j < b.len()
            invariant
                0 <= j && j <= b.len(),
                forall|k: int| 0 <= k && k < j as int ==> a[i as int] != b[k],
                i < a.len(),
            decreases
            b.len() - j
        {
            if a[i] == b[j] {
                return true;
            }
            j += 1;
        }
        i += 1;
    }

    false
}

fn main() {}

}