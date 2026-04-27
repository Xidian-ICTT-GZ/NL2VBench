use vstd::prelude::*;

verus! {

/// Computes element-wise modulo of two integer arrays: `result[i] = a[i] % b[i]`
fn element_wise_modulo(a: Vec<int>, b: Vec<int>) -> (result: Vec<int>)
    requires
        a.len() == b.len(),
        forall|i: int| 0 <= i && i < b.len() ==> b[i] != 0,
    ensures
        result.len() == a.len(),
        forall|i: int| 0 <= i && i < result.len() ==> result[i] == a[i] % b[i],
{
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            result.len() == i,
            forall|k: int| 0 <= k && k < i ==> result[k] == a[k] % b[k],
            b.len() == a.len(),
        decreases 
            a.len() - i
    {
        result.push(a[i] % b[i]);
        i += 1;
    }

    result
}

fn main() {}

}