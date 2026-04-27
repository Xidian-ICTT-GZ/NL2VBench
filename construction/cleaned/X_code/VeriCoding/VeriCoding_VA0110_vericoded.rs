use vstd::prelude::*;
verus! {
fn solve(r: i8, g: i8, b: i8) -> (result: i8){
    let ri: i16 = r as i16;
    let gi: i16 = g as i16;
    let bi: i16 = b as i16;
    let rg: i16 = ri + gi;
    let rb: i16 = ri + bi;
    let gb: i16 = gi + bi;
    let sum: i16 = rg + bi;
    let three: i16 = 3;
    let a3: i16 = sum / three;
    let m1: i16 = if a3 <= rg { a3 } else { rg };
    let m2: i16 = if m1 <= rb { m1 } else { rb };
    let m3: i16 = if m2 <= gb { m2 } else { gb };
    let res: i8 = m3 as i8;
    res
}
}
fn main() {}