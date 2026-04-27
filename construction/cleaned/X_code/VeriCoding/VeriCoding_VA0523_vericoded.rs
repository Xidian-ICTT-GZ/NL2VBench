use vstd::prelude::*;
verus! {
fn solve(n: i8, k: i8) -> (result: i8){
    let rem_i8: i8 = n % k;
    let complement_i8: i8 = k - rem_i8;
    let result: i8 = if rem_i8 <= complement_i8 { rem_i8 } else { complement_i8 };
    result
}
}
fn main() {}