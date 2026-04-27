use vstd::prelude::*;

verus! {

// Pure function to compute absolute value, returning a natural number (as `u64` or `int` in non-negative range)
spec fn abs(a: int) -> int {
    if a < 0 { -a } else { a }
}

// Method: aba
// Takes an array of integers and returns a new array containing the absolute values
fn aba(a: Vec<i32>) -> (b: Vec<i32>)
    requires
        a.len() < 1000000,
        forall|i: int| 0 <= i && i < a.len() as int ==> a[i] as int != -2147483648,
    ensures
        a.len() == b.len(),
        forall|x: int| 0 <= x && x < b.len() as int ==> b[x] == abs(a[x] as int) as i32,
{
    let mut b: Vec<i32> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            b.len() == i,
            forall|x: int| 0 <= x && x < i as int ==> b[x] == abs(a[x] as int) as i32,
            a.len() < 1000000,
            forall|x: int| 0 <= x && x < a.len() as int ==> a[x] as int != -2147483648,
        decreases 
            a.len() - i 
    {
        if a[i] < 0 {
            b.push(-a[i]);
        } else {
            b.push(a[i]);
        }
        i += 1;
    }

    b
}

fn main() {}

}