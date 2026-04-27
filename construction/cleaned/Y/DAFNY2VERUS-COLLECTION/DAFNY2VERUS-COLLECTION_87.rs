use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Pure function to compute the absolute value of an integer as a natural number
spec fn abs(x: int) -> int {
    if x < 0 { -x } else { x }
}

// Method: absx
// Takes an array of integers and returns a new array with absolute values
fn absx(x: Vec<i32>) -> (y: Vec<i32>)
    requires
        x.len() < 1000000,
        forall|i: int| 0 <= i && i < x.len() as int ==> x[i] as int != -2147483648,
    ensures
        y.len() == x.len(),
        forall|i: int| 0 <= i && i < y.len() as int ==> y[i] == abs(x[i] as int) as i32,
{
    let mut y: Vec<i32> = Vec::new();
    let mut j: usize = 0;

    while j < x.len()
        invariant
            0 <= j && j <= x.len(),
            y.len() == j,
            forall|i: int| 0 <= i && i < j as int ==> y[i] == abs(x[i] as int) as i32,
            x.len() < 1000000,
            forall|i: int| 0 <= i && i < x.len() as int ==> x[i] as int != -2147483648,
        decreases
            x.len() - j
    {
        if x[j] < 0 {
            y.push(-x[j]);
        } else {
            y.push(x[j]);
        }
        j += 1;
    }

    y
}

// Required empty main for Verus
fn main() {}

}