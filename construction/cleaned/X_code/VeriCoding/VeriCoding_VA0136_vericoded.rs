use vstd::prelude::*;
verus! {
fn solve(r1: i8, c1: i8, r2: i8, c2: i8) -> (result: Vec<i8>){
    let rook: i8 = if r1 == r2 && c1 == c2 {
        0
    } else if r1 == r2 || c1 == c2 {
        1
    } else {
        2
    };
    let row_diff: i8 = if r1 >= r2 { r1 - r2 } else { r2 - r1 };
    let col_diff: i8 = if c1 >= c2 { c1 - c2 } else { c2 - c1 };
    let bishop: i8 = if r1 == r2 && c1 == c2 {
        0
    } else if row_diff == col_diff {
        1
    } else if ((r1 + c1) % 2) == ((r2 + c2) % 2) {
        2
    } else {
        0
    };
    let row_diff_k: i8 = if r1 >= r2 { r1 - r2 } else { r2 - r1 };
    let col_diff_k: i8 = if c1 >= c2 { c1 - c2 } else { c2 - c1 };
    let king: i8 = if row_diff_k >= col_diff_k { row_diff_k } else { col_diff_k };
    let mut v = Vec::new();
    v.push(rook);
    v.push(bishop);
    v.push(king);
    v
}
}
fn main() {}