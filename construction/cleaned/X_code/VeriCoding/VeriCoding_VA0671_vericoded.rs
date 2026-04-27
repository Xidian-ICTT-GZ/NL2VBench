use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8, c: i8) -> (result: i8){
    if a == b && b == c {
        let r: i8 = 1;
        r
    } else if a == b || b == c || a == c {
        let r: i8 = 2;
        if a == b {
        } else if b == c {
        } else {
        }
        r
    } else {
        let r: i8 = 3;
        r
    }
}
}
fn main() {}