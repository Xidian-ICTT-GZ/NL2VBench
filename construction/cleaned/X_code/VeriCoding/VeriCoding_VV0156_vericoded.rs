use vstd::prelude::*;
verus! {
fn my_min(x: i32, y: i32) -> (result: i32){
    if x <= y {
        x
    } else {
        y
    }
}
}
fn main() {}