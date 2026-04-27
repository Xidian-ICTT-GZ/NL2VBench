use vstd::prelude::*;
verus! {
fn bitwise_xor(a: Vec<u8>, b: Vec<u8>) -> (result: Vec<u8>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < a.len()
    {
        result.push(a[i] ^ b[i]);
        i += 1;
    }
    result
}
}
fn main() {}