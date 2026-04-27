use vstd::prelude::*;
verus! {

fn solve(a: i8, b: i8, c: i8) -> (result: i8){
    let a0: i8;
    let a1: i8;
    let a2: i8;
    if a >= b && a >= c {
        if b >= c {
            a0 = a; a1 = b; a2 = c;
        } else {
            a0 = a; a1 = c; a2 = b;
        }
    } else if b >= a && b >= c {
        if a >= c {
            a0 = b; a1 = a; a2 = c;
        } else {
            a0 = b; a1 = c; a2 = a;
        }
    } else {
        if a >= b {
            a0 = c; a1 = a; a2 = b;
        } else {
            a0 = c; a1 = b; a2 = a;
        }
    }
    let gap1_i8: i8 = a0 - a1;
    let updated_smallest_i8: i8 = a2 + gap1_i8;
    let remaining_gap_i8: i8 = a0 - updated_smallest_i8;
    let ops_i8: i8 = gap1_i8 + (remaining_gap_i8 / 2) + ((remaining_gap_i8 % 2) * 2);
    let result: i8 = ops_i8;
    result
}
}
fn main() {}