use vstd::prelude::*;
verus! {
fn greater_equal(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < a.len()
    {
        let comparison = a[i as usize] >= b[i as usize];
        result.push(comparison);
        i += 1;
    }
    result
}
}
fn main() {}