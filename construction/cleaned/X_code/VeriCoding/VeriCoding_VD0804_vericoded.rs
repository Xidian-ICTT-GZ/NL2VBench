use vstd::prelude::*;
verus! {
fn count_equal_numbers(a: i32, b: i32, c: i32) -> (count: i32){
    if a == b {
        if b == c {
            3
        } else {
            2
        }
    } else if b == c {
        2
    } else if a == c {
        2
    } else {
        1
    }
}
fn main() {}
}