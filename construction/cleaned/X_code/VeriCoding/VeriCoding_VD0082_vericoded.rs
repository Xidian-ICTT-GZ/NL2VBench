use vstd::prelude::*;
verus! {
fn rotate(a: &[i32], offset: usize) -> (result: Vec<i32>){
    let n = a.len();
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let source_idx = ((i as u128 + offset as u128) % (n as u128)) as usize;
        result.push(a[source_idx]);
        i = i + 1;
    }
    result
}
}
fn main() {}