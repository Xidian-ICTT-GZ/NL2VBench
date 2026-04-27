use vstd::prelude::*;
verus! {
fn append(a: &Vec<i32>, b: i32) -> (c: Vec<i32>){
    let mut c = a.clone();
    c.push(b);
    c
}
fn main() {
}
}