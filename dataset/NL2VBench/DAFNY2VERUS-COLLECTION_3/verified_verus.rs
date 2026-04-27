use vstd::prelude::*;

verus! {

// Method: CountToAndReturnN
// Counts from 0 to n and returns n
fn count_to_and_return_n(n: u64) -> (r: u64)
    requires
        n >= 0,  // Always true for u64, but preserved in spec
    ensures
        r == n,
{
    let mut i: u64 = 0;
    while i < n
        invariant
            0 <= i && i <= n,
         decreases
            n - i 
    {
        i = i + 1;
    }
    i  // return i
}

fn main() {}

}