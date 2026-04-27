use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: CubeElements
// Takes a Vec<int> and returns a new Vec<int> where each element is cubed.
fn cube_elements(a: Vec<int>) -> (cubed: Vec<int>)
    ensures
        cubed.len() == a.len(),
        forall|i: int| 0 <= i < a.len() ==> cubed[i] == a[i] * a[i] * a[i],
{
    let mut cubed_array: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i as int && i as int <= a.len() as int,
            cubed_array.len() == i,
            cubed_array.len() == i && i <= a.len(),
            forall|k: int| 0 <= k < i as int ==> cubed_array[k] == a[k] * a[k] * a[k],
         decreases 
            a.len() - i
    {
        let val = a[i];
        cubed_array.push(val * val * val);
        i += 1;
    }

    cubed_array
}

// Empty main function as required
fn main() {}

}