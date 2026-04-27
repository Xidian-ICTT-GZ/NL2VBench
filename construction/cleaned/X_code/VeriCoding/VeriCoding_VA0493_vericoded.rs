use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8) -> (result: i8){
    let a_wide = a as i32;
    let b_wide = b as i32;
    if a_wide > 2 * b_wide {
        (a_wide - 2 * b_wide) as i8
    } else {
        0
    }
}
}
fn main() {}