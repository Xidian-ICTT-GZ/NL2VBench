use vstd::math::*;
use vstd::prelude::*;

fn main() {}

verus! {

fn difference_max_min(arr: &Vec<i32>) -> (diff: i32)
    requires
        arr.len() > 0,
        forall|i: int| 0 <= i < arr.len() ==> i32::MIN / 2 <= #[trigger] arr[i] <= i32::MAX / 2,
    ensures
        forall|i: int, j: int| 0 <= i < arr.len() && 0 <= j < arr.len() ==> arr[i] - arr[j] <= diff,
        diff >= 0,
{
    let mut min_val = arr[0];
    let mut max_val = arr[0];
    let mut index = 1;

    while index < arr.len()
        invariant
            1 <= index <= arr.len(),
            min_val <= max_val,
            forall|k: int| 0 <= k < index ==> min_val <= #[trigger] arr[k] && #[trigger] arr[k] <= max_val,
            exists|k: int| 0 <= k < index && arr[k] == min_val,
            exists|k: int| 0 <= k < index && arr[k] == max_val,
        decreases arr.len() - index,
    {
        if (arr[index] <= min_val) {
            min_val = arr[index];
        } else if (arr[index] > max_val) {
            max_val = arr[index];
        }
        index += 1;
    }
    max_val - min_val
}

} // verus!