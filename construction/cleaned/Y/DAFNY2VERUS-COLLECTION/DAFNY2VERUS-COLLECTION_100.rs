use vstd::prelude::*;

verus! {

// Function: BinarySearch
// Performs binary search on a sorted array `a` to find the leftmost position where `key` can be inserted
// such that all elements to the left are less than `key`, and all to the right are greater or equal.
fn binary_search(a: &Vec<i32>, key: i32) -> (n: usize)
    requires
        // Array is sorted in non-decreasing order
        forall| i: int, j: int | 0 <= i < j < a.len() ==> a[i] <= a[j],
    ensures
        0 <= n && n <= a.len(),
        forall| i: int | 0 <= i && i < n ==> a[i] < key,
        forall| i: int | n <= i && i < a.len() ==> key <= a[i],
{
    let mut lo: usize = 0;
    let mut hi: usize = a.len();

    while lo < hi
        invariant
            0 <= lo && lo <= hi && hi <= a.len(),
            forall| i: int | 0 <= i && i < lo ==> a[i] < key,
            forall| i: int | hi <= i && i < a.len() ==> key <= a[i],
            forall| i: int, j: int | 0 <= i < j < a.len() ==> a[i] <= a[j],
        decreases
            hi - lo
    {
        // Compute mid safely to avoid overflow
        let mid = lo + (hi - lo) / 2;

        if a[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo // return lo as the insertion index
}

fn main() {}

}