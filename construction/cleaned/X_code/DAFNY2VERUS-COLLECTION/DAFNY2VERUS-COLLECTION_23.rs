use vstd::prelude::*;
verus! {
fn element_wise_modulo(a: Vec<int>, b: Vec<int>) -> (result: Vec<int>){
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        result.push(a[i] % b[i]);
        i += 1;
    }
    result
}
fn main() {}
}