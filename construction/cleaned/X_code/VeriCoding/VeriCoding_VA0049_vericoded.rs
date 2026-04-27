use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: i8){
    if n % 2i8 != 0i8 {
        0i8
    } else {
        let q: i8 = n / 4i8;
        if n % 4i8 == 2i8 {
            q
        } else {
            q - 1i8
        }
    }
}
}
fn main() {}