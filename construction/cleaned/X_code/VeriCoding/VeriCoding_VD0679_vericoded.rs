use vstd::prelude::*;
verus! {
fn min_of_three(a: i32, b: i32, c: i32) -> (min: i32){
    if a <= b {
        if a <= c {
            a
        } else {
            c
        }
    } else {
        if b <= c {
            b
        } else {
            c
        }
    }
}
fn main() {
}
}