use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn append_array(a: Vec<i32>, b: Vec<i32>) -> (c: Vec<i32>){
    let mut c: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        c.push(a[i]);
        i += 1;
    }
    i = 0;
    while i < b.len()
    {
        c.push(b[i]);
        i += 1;
    }
    c
}
fn main() {}
}