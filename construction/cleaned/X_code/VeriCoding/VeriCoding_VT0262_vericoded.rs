use vstd::prelude::*;
verus! {
fn numpy_greater_equal(x1: Vec<i8>, x2: Vec<i8>) -> (result: Vec<bool>){
    let n = x1.len();
    let mut result: Vec<bool> = Vec::new();
    let mut i: usize = 0usize;
    while i < n
    {
        let b: bool = x1[i] >= x2[i];
        result.push(b);
        i = i + 1;
    }
    result
}
}
fn main() {}