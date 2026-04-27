use vstd::prelude::*;
verus! {
fn ones(n: usize) -> (result: Vec<i8>){
    let mut v: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        v.push(1i8);
        i = i + 1;
    }
    v
}
}
fn main() {}