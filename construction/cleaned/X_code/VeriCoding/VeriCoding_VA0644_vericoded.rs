use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8) -> (result: i8){
    let result: i8;
    if a > b {
        result = a - 1;
    } else {
        result = a;
    }
    result
}
}
fn main() {}