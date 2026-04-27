use vstd::prelude::*;
verus! {
fn compare(a: i32, b: i32) -> (result: bool){
    if a == b {
        true
    } else {
        false
    }
}
}
fn main() {}