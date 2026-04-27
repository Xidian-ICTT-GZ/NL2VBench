use vstd::prelude::*;

verus! {

// Function: main
// Decrements k by n and returns the result
fn main_func(n: i32, k: i32) -> (k_out: i32)
    requires
        n > 0,
        k > n,
    ensures
        k_out >= 0,
        k_out == k - n,  // This follows from behavior, though not required in post
{
    let mut k_out = k;
    let mut j: i32 = 0;

    while j < n
        invariant
            0 <= j && j <= n,
            k_out == k.wrapping_sub(j),
        decreases
            n - j
    {
        k_out = k_out.wrapping_sub(1);
        j = j + 1;
    }

    k_out
}

} // end verus!

// Required empty main function
fn main() {}