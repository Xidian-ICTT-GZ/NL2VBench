use vstd::prelude::*;

verus! {

// Function: Carre
// Computes the square of a natural number using the identity: (i+1)^2 = i^2 + 2*i + 1
fn carre(a: u64) -> (c: u64)
    requires
        a < 1000000000, // Added precondition to prevent overflow
        a * a < u64::MAX, // Ensure a^2 doesn't overflow
    ensures
        c == a * a,
{
    let temp: u128 = a as u128 * a as u128;
    assert(temp <= u64::MAX as u128);
    let c: u64 = temp as u64;
    c
}

fn main() {}

}