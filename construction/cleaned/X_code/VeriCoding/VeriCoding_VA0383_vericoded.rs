use vstd::prelude::*;
verus! {
fn check_lamp_arrangement(r: i8, g: i8, b: i8) -> (result: bool){
    let rr_i32: i32 = r as i32;
    let gg_i32: i32 = g as i32;
    let bb_i32: i32 = b as i32;
    let max32: i32 = if rr_i32 >= gg_i32 && rr_i32 >= bb_i32 { rr_i32 } else if gg_i32 >= rr_i32 && gg_i32 >= bb_i32 { gg_i32 } else { bb_i32 };
    let total32: i32 = rr_i32 + gg_i32 + bb_i32;
    let result: bool = 2 * max32 <= total32 + 1;
    result
}
}
fn main() {}