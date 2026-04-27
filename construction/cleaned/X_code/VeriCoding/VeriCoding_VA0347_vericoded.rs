use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8) -> (result: (i8, i8)){
    let s16: i16 = (a as i16) + (b as i16);
    let d16: i16 = (a as i16) - (b as i16);
    let x: i8 = (s16 / 2) as i8;
    let y: i8 = (d16 / 2) as i8;
    (x, y)
}
}
fn main() {}