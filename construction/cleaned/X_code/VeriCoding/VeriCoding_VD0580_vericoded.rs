use vstd::prelude::*;
verus! {
fn append_array(a: &[i32], b: &[i32]) -> (c: Vec<i32>){
    let mut c: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        c.push(a[i]);
        i += 1;
    }
    let mut j: usize = 0;
    while j < b.len()
    {
        c.push(b[j]);
        j += 1;
    }
    c
}
fn main() {
}
}