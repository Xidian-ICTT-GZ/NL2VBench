use vstd::prelude::*;
verus! {
fn min(x: int, y: int) -> (z: int){
    if x <= y {
        x
    } else {
        y
    }
}
fn main() {}
}