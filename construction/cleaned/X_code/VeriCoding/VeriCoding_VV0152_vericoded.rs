use vstd::prelude::*;
verus! {
fn id_i32(x: i32) -> i32 { x }
fn append(a: &Vec<i32>, b: i32) -> (result: Vec<i32>){
    let mut r = a.clone();
    r.push(b);
    r
}
}
fn main() {}