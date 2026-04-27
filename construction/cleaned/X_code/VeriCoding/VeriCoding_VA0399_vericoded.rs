use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8) -> (result: i8){
    let res: i8 = 6i8 - a - b;
    if a == 1i8 {
        if b == 2i8 {
        } else if b == 3i8 {
        } else {
        }
    } else if a == 2i8 {
        if b == 1i8 {
        } else if b == 3i8 {
        } else {
        }
    } else {
        if b == 1i8 {
        } else if b == 2i8 {
        } else {
        }
    }
    res
}
}
fn main() {}