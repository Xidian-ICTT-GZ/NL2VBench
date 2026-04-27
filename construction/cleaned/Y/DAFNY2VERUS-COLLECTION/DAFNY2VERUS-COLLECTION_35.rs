use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method: MaxArray
// Returns the maximum value in a non-empty array of integers
fn max_array(a: &Vec<i32>) -> (max: i32)
    requires
        a.len() > 0,
    ensures
        forall|i: int| 0 <= i < a.len() ==> a[i] <= max,
        exists|i: int| 0 <= i < a.len() && a[i] == max,
{
    let mut i: usize = 1;
    let mut max = a[0];

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|j: int| 0 <= j < i as int ==> a[j] <= max,
            exists|j: int| 0 <= j < i as int && a[j] == max,
        decreases a.len() - i
    {
        if a[i] > max {
            max = a[i];
        }
        i += 1;
    }

    max
}

} // end verus!

// Test in Rust-level main
fn main() {
    // Test case: arr = [-11, 2, 42, -4]
    let arr = vec![-11, 2, 42, -4];
    let res = max_array(&arr);

    // Check original array values
    assert!(arr[0] == -11 && arr[1] == 2 && arr[2] == 42 && arr[3] == -4);
    // Check result
    assert!(res == 42);

    println!("All tests passed!");
}