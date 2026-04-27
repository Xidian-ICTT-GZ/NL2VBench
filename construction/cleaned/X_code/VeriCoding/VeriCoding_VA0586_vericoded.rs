use vstd::prelude::*;
verus! {
fn solve(x: i8, y: i8, z: i8) -> (result: i8){
    let numerator: i8 = x - z;
    let denominator: i8 = y + z;
    let result: i8 = numerator / denominator;
    result
}
}
fn main() {}