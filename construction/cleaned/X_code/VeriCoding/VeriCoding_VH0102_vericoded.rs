use vstd::prelude::*;
verus! {
fn choose_num(x: i8, y: i8) -> (result: i8){
    if x > y {
        return -1;
    }
    let mut cand: i8;
    if y % 2 == 0 {
        cand = y;
    } else {
        cand = y - 1;
    }
    if cand >= x {
        cand
    } else {
        -1
    }
}
}
fn main() {}