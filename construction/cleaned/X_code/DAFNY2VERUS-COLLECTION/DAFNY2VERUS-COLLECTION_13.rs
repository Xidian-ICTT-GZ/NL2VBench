use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn pairwise_addition(a: Vec<int>) -> (result: Vec<int>){
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len() / 2
    {
        let sum = a[2 * i] + a[2 * i + 1];
        result.push(sum);
        i += 1;
    }
    result
}
fn main() {}
}