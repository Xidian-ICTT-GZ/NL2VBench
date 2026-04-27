use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8, k: i8) -> (result: (i8, i8)){
    let tak: i8 = if a >= k { a - k } else { 0i8 };
    let rem: i8 = if a >= k { 0i8 } else { k - a };
    let aoki: i8 = if a >= k {
        b
    } else if rem < b {
        b - rem
    } else {
        0i8
    };
    (tak, aoki)
}
}
fn main() {}