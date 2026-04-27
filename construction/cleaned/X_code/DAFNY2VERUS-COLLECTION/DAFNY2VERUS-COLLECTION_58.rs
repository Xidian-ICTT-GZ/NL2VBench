use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn reverse(a: &[char]) -> (b: Vec<char>){
    let mut b: Vec<char> = Vec::with_capacity(a.len());
    let mut i: usize = 0;
    while i < a.len()
    {
        let index_in_a = a.len() - 1 - i;
        b.push(a[index_in_a]);
        i += 1;
    }
    b
}
fn main() {}
}