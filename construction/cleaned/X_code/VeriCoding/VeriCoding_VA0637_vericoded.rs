use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: bool){
    let m: i32 = n as i32;
    let d1 = m / 1000;
    let d2 = (m / 100) % 10;
    let d3 = (m / 10) % 10;
    let d4 = m % 10;
    let result = (d1 == d2 && d2 == d3) || (d2 == d3 && d3 == d4);
    result
}
}
fn main() {}