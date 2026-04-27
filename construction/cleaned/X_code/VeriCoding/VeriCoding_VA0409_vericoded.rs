use vstd::prelude::*;
verus! {
fn solve_rival_distance(n: i8, x: i8, a: i8, b: i8) -> (result: i8){
    let d: i8 = if a >= b { a - b } else { b - a };
    let n_minus_1: i8 = n - 1;
    let cap: i8 = n_minus_1 - d;
    let result: i8;
    if x <= cap {
        result = d + x;
    } else {
        result = n_minus_1;
    }
    result
}
}
fn main() {}