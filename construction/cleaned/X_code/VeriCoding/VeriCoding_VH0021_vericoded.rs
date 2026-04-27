use vstd::prelude::*;
verus! {
fn largest_divisor(n: i8) -> (d: i8){
    let mut i = n - 1;
    while i > 0
    {
        if n % i == 0 {
            return i;
        }
        i = i - 1;
    }
    unreached()
}
}
fn main() {}