use vstd::prelude::*;
verus! {
fn largest_power_of_two_le(n: i8) -> (z: i8){
    let mut z: i8 = 1;
    while z <= n / 2
    {
        z = z * 2;
    }
    z
}
fn solve(n: i8) -> (result: i8){
    if n % 2 != 0 {
        let r: i8 = (n - 1) / 2;
        return r;
    }
    let z: i8 = largest_power_of_two_le(n);
    let r: i8 = (n - z) / 2;
    r
}
}
fn main() {}