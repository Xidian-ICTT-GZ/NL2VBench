use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Finds the maximum value in a non-empty array of integers (modeling reals)
fn find_max(a: &Vec<i32>) -> (max: i32)
    requires
        a.len() > 0,
    ensures
        exists|k: int| 0 <= k && k < a.len() && max == a[k],
        forall|k: int| 0 <= k && k < a.len() ==> max >= a[k],
{
    let mut max = a[0];
    let mut i = 1;

    while i < a.len()
        invariant
            1 <= i <= a.len(),
            exists|k: int| 0 <= k && k < i && max == a[k],
            forall|k: int| 0 <= k && k < i ==> max >= a[k],
        decreases
            a.len() - i 
    {
        if a[i] > max {
            max = a[i];
        }
        i += 1;
    }

    max
}

} // end verus!

// Rust-level test cases using assertions
fn main() {
    // Test case 1: [1.0, 2.0, 3.0] → 3.0
    let a1 = vec![1, 2, 3];
    let m1 = find_max(&a1);
    assert!(m1 == 3);
    assert!(m1 == a1[2]);

    // Test case 2: [3.0, 2.0, 1.0] → 3.0
    let a2 = vec![3, 2, 1];
    let m2 = find_max(&a2);
    assert!(m2 == 3);
    assert!(m2 == a2[0]);

    // Test case 3: [2.0, 3.0, 1.0] → 3.0
    let a3 = vec![2, 3, 1];
    let m3 = find_max(&a3);
    assert!(m3 == 3);
    assert!(m3 == a3[1]);

    // Test case 4: [1.0, 2.0, 2.0] → 2.0
    let a4 = vec![1, 2, 2];
    let m4 = find_max(&a4);
    assert!(m4 == 2);
    assert!(m4 == a4[1] || m4 == a4[2]);

    // Test case 5: [1.0] → 1.0
    let a5 = vec![1];
    let m5 = find_max(&a5);
    assert!(m5 == 1);
    assert!(m5 == a5[0]);

    // Test case 6: [1.0, 1.0, 1.0] → 1.0
    let a6 = vec![1, 1, 1];
    let m6 = find_max(&a6);
    assert!(m6 == 1);
    assert!(m6 == a6[0]);

    println!("All tests passed!");
}