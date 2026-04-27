use vstd::prelude::*;
verus! {
fn is_divisible_by_11(n: i32) -> (result: bool){
    let result: bool = n % 11 == 0;
    result
}
}
fn main() {}