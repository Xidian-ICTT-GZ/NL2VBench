use vstd::prelude::*;
verus! {
fn solve(n: i8, k: i8) -> (result: i8){
    let res: i8;
    if n % k == 0i8 {
        res = 0i8;
    } else {
        res = 1i8;
    }
    res
}
}
fn main() {}