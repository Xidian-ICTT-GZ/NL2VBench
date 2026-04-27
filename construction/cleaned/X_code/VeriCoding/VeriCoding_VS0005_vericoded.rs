use vstd::prelude::*;
verus! {
fn bitwise_and(a: Vec<u8>, b: Vec<u8>) -> (result: Vec<u8>){
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let val = a[i] & b[i];
        result.push(val);
        i = i + 1;
    }
    result
}
}
fn main() {}