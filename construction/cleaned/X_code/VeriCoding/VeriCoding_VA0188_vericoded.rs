use vstd::prelude::*;
verus! {
fn max(x: i8, y: i8) -> (res: i8){
    if x >= y { x } else { y }
}
fn max3(x: i8, y: i8, z: i8) -> (res: i8){
    max(x, max(y, z))
}
fn solve(a: i8, b: i8, c: i8) -> (result: i8){
    let max_val = max3(a, b, c);
    let a_w = a as i16;
    let b_w = b as i16;
    let c_w = c as i16;
    let max_val_w = max_val as i16;
    let sum_of_other_two_w = a_w + b_w + c_w - max_val_w;
    if sum_of_other_two_w > max_val_w {
        0i8
    } else {
        let ops = max_val_w - sum_of_other_two_w + 1;
        ops as i8
    }
}
}
fn main() {}