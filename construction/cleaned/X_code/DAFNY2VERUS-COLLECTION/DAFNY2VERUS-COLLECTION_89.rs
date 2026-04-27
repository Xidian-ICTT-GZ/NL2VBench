use vstd::prelude::*;
verus! {
fn aba(a: Vec<i32>) -> (b: Vec<i32>){
    let mut b: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] < 0 {
            b.push(-a[i]);
        } else {
            b.push(a[i]);
        }
        i += 1;
    }
    b
}
fn main() {}
}