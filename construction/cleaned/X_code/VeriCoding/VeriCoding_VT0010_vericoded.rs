use vstd::prelude::*;
verus! {
fn empty(n: u8) -> (result: Vec<f64>){
    let mut v: Vec<f64> = Vec::new();
    while v.len() < n as usize
    {
        v.push(0.0);
    }
    v
}
}
fn main() {}