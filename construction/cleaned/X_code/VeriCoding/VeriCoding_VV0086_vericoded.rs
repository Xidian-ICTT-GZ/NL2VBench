use vstd::prelude::*;
verus! {
fn min_i32(a: i32, b: i32) -> (res: i32){
    if a <= b { a } else { b }
}
fn min_of_three(a: i32, b: i32, c: i32) -> (result: i32){
    let m = min_i32(a, b);
    let r = min_i32(m, c);
    r
}
}
fn main() {}