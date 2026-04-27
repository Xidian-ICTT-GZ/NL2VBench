use vstd::prelude::*;
verus! {
fn solve(n: i8, m: i8) -> (result: i8){
    let n32: i32 = n as i32;
    let m32: i32 = m as i32;
    let direct_groups: i32 = if n32 < m32 / 2 { n32 } else { m32 / 2 };
    let remaining_c_pieces: i32 = m32 - direct_groups * 2;
    let additional_groups: i32 = remaining_c_pieces / 4;
    let total32: i32 = direct_groups + additional_groups;
    let result: i8;
    result = total32 as i8;
    result
}
}
fn main() {}