use vstd::prelude::*;
verus! {
fn clamp_to_i8(x: i64) -> i8 {
    if x < -128 { -128i8 } else if x > 127 { 127i8 } else { x as i8 }
}
fn solve(input: Vec<i8>) -> (result: Vec<i8>){
    if input.len() < 3 {
        return Vec::<i8>::new();
    }
    let a = input[0] as i64;
    let b = input[1] as i64;
    let c = input[2] as i64;
    let available = a - b;
    let mut remaining = c - available;
    if remaining < 0 {
        remaining = 0;
    }
    let out = clamp_to_i8(remaining);
    let mut res: Vec<i8> = Vec::new();
    res.push(out);
    res
}
}
fn main() {}