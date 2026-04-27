use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Verified function to compute the maximum difference between any two elements in the array
fn MaxDifference(a: Vec<int>) -> (diff: int)
    requires a.len() > 1,
    ensures forall|i: int, j: int| 
        0 <= i && i < a.len() && 0 <= j && j < a.len() ==> a[i] - a[j] <= diff,
{
    let mut minVal = a[0];
    let mut maxVal = a[0];

    let mut i = 1;
    while i < a.len() 
        invariant 
            1 <= i <= a.len(),
            minVal <= maxVal,
            forall|k: int| 0 <= k < i ==> minVal <= a[k] && a[k] <= maxVal,
        decreases
            a.len()-i
    {
        if a[i] < minVal {
            minVal = a[i];
        } else if a[i] > maxVal {
            maxVal = a[i];
        }
        i += 1;
    }

    maxVal - minVal
}

// Required empty main function
fn main() {}
}