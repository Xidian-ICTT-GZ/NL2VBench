use vstd::prelude::*;
verus! {

fn last_digit(n: i32) -> (result: i32){
    n % 10
}
}
fn main() {}