use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: Vec<char>){
    let mut result: Vec<char> = Vec::new();
    result.push('A');
    result.push('B');
    result.push('C');
    let ni: i32 = n as i32;
    let d1_i32: i32 = ni / 100;
    let d2_i32: i32 = (ni / 10) % 10;
    let d3_i32: i32 = ni % 10;
    let c1: char = ('0' as u8 + (d1_i32 as u8)) as char;
    let c2: char = ('0' as u8 + (d2_i32 as u8)) as char;
    let c3: char = ('0' as u8 + (d3_i32 as u8)) as char;
    result.push(c1);
    result.push(c2);
    result.push(c3);
    result
}
}
fn main() {}