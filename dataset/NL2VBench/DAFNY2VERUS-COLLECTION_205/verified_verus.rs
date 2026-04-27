use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method: replace
// Modifies the array by replacing every element greater than `k` with -1
fn replace(arr: &mut Vec<i32>, k: i32)
    ensures
        forall|i: int| 0 <= i < arr.len() ==> #[trigger] old(arr)[i] > k ==> arr[i] == -1,
        forall|i: int| 0 <= i < arr.len() ==> #[trigger] old(arr)[i] <= k ==> arr[i] == old(arr)[i],
{
    let mut i: usize = 0;

    while i < arr.len()
        invariant
            0 <= i && i <= arr.len(),
            forall|j: int| 0 <= j < i as int ==> #[trigger] old(arr)[j] > k ==> arr[j] == -1,
            forall|j: int| 0 <= j < i as int ==> #[trigger] old(arr)[j] <= k ==> arr[j] == old(arr)[j],
            forall|j: int| i as int <= j && j < arr.len() ==> arr[j] == old(arr)[j],
        decreases arr.len() - i
    {
        if arr[i] > k {
            arr[i] = -1;
        }
        i += 1;
    }
}

fn main() {}

}