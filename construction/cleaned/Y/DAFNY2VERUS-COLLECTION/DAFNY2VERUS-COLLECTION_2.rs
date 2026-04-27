#![allow(unused)]
use vstd::prelude::*;

verus! {

// Define the max function (ghost, since it's used in spec)
spec fn max(x: u64, y: u64) -> u64 {
    if x < y { y } else { x }
}

// Method implementation
fn slow_max(a: u64, b: u64) -> (z: u64)
    ensures
        z == max(a, b),
{
    let mut z: u64 = 0;
    let mut x: u64 = a;
    let mut y: u64 = b;

    while z < x && z < y
        invariant
            x >= 0,
            y >= 0,
            z == a - x && z == b - y,
            a - x == b - y,
        decreases x, y
    {
        z = z + 1;
        x = x - 1;
        y = y - 1;
    }

    if x <= y {
        b
    } else {
        a
    }
}

// Required empty main
fn main() {}
}