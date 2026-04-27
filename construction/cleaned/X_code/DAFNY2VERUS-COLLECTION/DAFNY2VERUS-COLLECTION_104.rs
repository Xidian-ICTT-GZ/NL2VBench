use vstd::prelude::*;
verus! {
fn rotate(a: Vec<int>, offset: usize) -> (b: Vec<int>){
    let mut b: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let idx = (i + offset) % a.len();
        b.push(a[idx]);
        i += 1;
    }
    b
}
fn main() {}
}