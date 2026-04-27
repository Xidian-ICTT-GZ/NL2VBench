use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn square_elements(a: Vec<int>) -> (squared: Vec<int>){
    let mut squared = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        squared.push(a[i] * a[i]);
        i += 1;
    }
    squared
}
fn main() {}
}