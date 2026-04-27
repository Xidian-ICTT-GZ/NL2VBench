use vstd::prelude::*;
verus! {

fn gcd_i(m: int, n: int) -> (d: int){
    let mut a = m;
    let mut b = n;
    while a != b
    {
        if a > b {
            let old_a = a;
            let old_b = b;
            a = old_a - old_b;
        } else {
            let old_a = a;
            let old_b = b;
            b = old_b - old_a;
        }
    }
    a
}
fn main() {
}
}