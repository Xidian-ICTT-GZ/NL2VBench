use vstd::prelude::*;
verus! {
fn gcd_calc(m: u32, n: u32) -> (res: u32){
    let mut a = m;
    let mut b = n;
    while a != b
    {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    a
}
fn main() {}
}