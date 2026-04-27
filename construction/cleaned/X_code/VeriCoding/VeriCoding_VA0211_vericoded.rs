use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: i8){
    if n % 3 == 0 {
        2 * (n / 3)
    } else {
        2 * (n / 3) + 1
    }
}
}
fn main() {}