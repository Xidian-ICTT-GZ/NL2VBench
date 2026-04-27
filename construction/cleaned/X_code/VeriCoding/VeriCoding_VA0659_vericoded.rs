use vstd::prelude::*;
verus! {
fn solve(m: i8) -> (result: i8){
    let mut curr: i8 = m;
    let mut result: i8 = 0i8;
    while curr < 48i8
    {
        result = result + 1i8;
        curr = curr + 1i8;
    }
    result
}
}
fn main() {}