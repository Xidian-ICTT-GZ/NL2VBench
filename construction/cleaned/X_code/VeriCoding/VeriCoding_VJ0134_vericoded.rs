use vstd::prelude::*;
verus! {
fn abs(x: i32) -> (result: i32){
    if x >= 0 {
        x
    } else {
        -x
    }
}
}
fn main() {}