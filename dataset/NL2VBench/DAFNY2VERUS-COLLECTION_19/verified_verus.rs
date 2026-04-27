use vstd::prelude::*;

verus! {

// Function: HasOnlyOneDistinctElement
// Checks whether all elements in the array are equal
fn has_only_one_distinct_element(a: &Vec<i32>) -> (result: bool)
    ensures
        result ==> forall|i: int, j: int| 0 <= i && i < a.len() && 0 <= j && j < a.len() ==> a[i] == a[j],
        !result ==> exists|i: int, j: int| 0 <= i && i < a.len() && 0 <= j && j < a.len() && a[i] != a[j],
{
    if a.len() == 0 {
        return true;
    }

    let first_element = a[0];
    let mut result = true;
    let mut i = 1;

    while i < a.len()
        invariant
            1 <= i && i <= a.len(),
            result ==> forall|k: int| 0 <= k && k < i ==> a[k] == first_element,
            !result ==> exists|k: int| 0 <= k && k < i && a[k] != first_element,
         decreases 
            a.len() - i
    {
        if a[i] != first_element {
            result = false;
        }
        i += 1;
    }

    result
}

fn main() {}

}