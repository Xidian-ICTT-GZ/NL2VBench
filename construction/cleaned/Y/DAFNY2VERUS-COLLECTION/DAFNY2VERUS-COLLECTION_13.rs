use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Method: PairwiseAddition
// Takes an array `a` of even length and returns a new array `result` where each element
// is the sum of consecutive pairs of elements from `a`: result[i] = a[2*i] + a[2*i+1]
fn pairwise_addition(a: Vec<int>) -> (result: Vec<int>)
    requires
        a.len() % 2 == 0,
    ensures
        result.len() == a.len() / 2,
        forall|i: int| 0 <= i < result.len() ==> result[i] == a[2 * i] + a[2 * i + 1],
{
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len() / 2
        invariant
            0 <= i && i <= a.len() / 2,
            result.len() == i,
            forall|k: int| 0 <= k < i ==> result[k] == a[2 * k] + a[2 * k + 1],
        decreases
            a.len()/2-i
    {
        let sum = a[2 * i] + a[2 * i + 1];
        result.push(sum);
        i += 1;
    }

    result
}

fn main() {}

}