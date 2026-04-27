use vstd::prelude::*;
verus! {
fn numpy_not_equal(x1: Vec<i8>, x2: Vec<i8>) -> (result: Vec<bool>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < x1.len()
    {
        let r = x1[i] != x2[i];
        result.push(r);
        i = i + 1;
    }
    result
}
}
fn main() {}