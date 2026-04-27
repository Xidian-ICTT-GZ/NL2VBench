use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: i8){
    let result: i8 = if n % 2 == 0 { n / 2 } else { n / 2 - n };
    result
}
}
fn main() {}