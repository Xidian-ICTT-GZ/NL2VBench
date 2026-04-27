use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn array_up_to_n(n: usize) -> (a: Vec<usize>){
    let mut a: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        a.push(i);
        i += 1;
    }
    a
}
fn main() {}
}