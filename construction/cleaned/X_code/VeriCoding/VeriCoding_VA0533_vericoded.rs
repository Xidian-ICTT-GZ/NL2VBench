use vstd::prelude::*;
verus! {
fn solve(a1: i8, a2: i8, a3: i8) -> (result: i8){
    let maxv: i8 = if a1 >= a2 && a1 >= a3 { a1 } else if a2 >= a3 { a2 } else { a3 };
    let minv: i8 = if a1 <= a2 && a1 <= a3 { a1 } else if a2 <= a3 { a2 } else { a3 };
    let result: i8 = maxv - minv;
    result
}
}
fn main() {}