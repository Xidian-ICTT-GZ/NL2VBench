use vstd::prelude::*;
verus! {
fn carre(a: u64) -> (c: u64){
    let temp: u128 = a as u128 * a as u128;
    let c: u64 = temp as u64;
    c
}
fn main() {}
}