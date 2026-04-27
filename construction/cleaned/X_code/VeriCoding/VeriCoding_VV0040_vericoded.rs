use vstd::prelude::*;
verus! {
fn max2_exec(x: i32, y: i32) -> (r: i32){
    if x >= y {
        x
    } else {
        y
    }
}
fn max_of_three(a: i32, b: i32, c: i32) -> (result: i32){
    let m = max2_exec(a, b);
    let r = max2_exec(m, c);
    r
}
}
fn main() {}