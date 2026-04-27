use vstd::prelude::*;
verus! {
    
fn solve(a: i8, b: i8, c: i8, d: i8) -> (result: i8){
    let m1_i8: i8 = if b < d { b } else { d };
    let M1_i8: i8 = if a > c { a } else { c };
    let overlap_i8: i8 = m1_i8 - M1_i8;
    let res_i8: i8 = if overlap_i8 > 0 { overlap_i8 } else { 0 };
    res_i8
}
}
fn main() {}