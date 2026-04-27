use vstd::prelude::*;
verus! {
fn solve(x: i32) -> (result: i32){
    let len: i32 = if x <= 9 { 1 } else if x <= 99 { 2 } else if x <= 999 { 3 } else { 4 };
    let digit: i32 = if x <= 9 { x } else if x <= 99 { x / 11 } else if x <= 999 { x / 111 } else { x / 1111 };
    let prev: i32 = if digit == 1 { 0 } else { (digit - 1) * 10 };
    let curr: i32 = (len * (len + 1)) / 2;
    let result0: i32 = prev + curr;
    result0
}
}
fn main() {}