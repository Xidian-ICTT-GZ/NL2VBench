use vstd::prelude::*;
verus! {
fn build_output_vec(temp: i8) -> (v: Vec<u8>){
    if temp >= 30 {
        let v1: Vec<u8> = vec![89u8, 101u8, 115u8, 10u8];
        v1
    } else {
        let v2: Vec<u8> = vec![78u8, 111u8, 10u8];
        v2
    }
}
fn solve(x: i8) -> (result: Vec<u8>){
    let result = build_output_vec(x);
    result
}
}
fn main() {}