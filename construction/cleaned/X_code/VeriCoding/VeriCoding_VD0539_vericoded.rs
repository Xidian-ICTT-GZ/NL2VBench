use vstd::prelude::*;
verus! {
fn reverse(a: &Vec<i32>) -> (a_rev: Vec<i32>){
    let mut a_rev = Vec::with_capacity(a.len());
    let mut i = 0;
    while i < a.len()
    {
        a_rev.push(a[a.len() - i - 1]);
        i += 1;
    }
    a_rev
}
fn main() {}
}