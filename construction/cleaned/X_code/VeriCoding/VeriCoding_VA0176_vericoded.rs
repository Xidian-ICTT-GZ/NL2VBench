use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8) -> (result: (i8, i8)){
    let days_different: i8 = if a < b { a } else { b };
    let remaining: i8 = if a > b { a - days_different } else { b - days_different };
    let days_same: i8 = remaining / 2;
    (days_different, days_same)
}
}
fn main() {}