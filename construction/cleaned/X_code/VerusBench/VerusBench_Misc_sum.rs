#[allow(unused_imports)]
use vstd::prelude::*;

fn main() {}

verus! {

fn compute_arith_sum(n: u64) -> (sum: u64){
    let mut i: u64 = 0;
    let mut sum: u64 = 0;
    while i < n
    {
        i = i + 1;
        sum = sum + i;
    }
    sum
}

} // verus!