use vstd::prelude::*;
verus! {
fn bitwise_and(x1: &Vec<u8>, x2: &Vec<u8>) -> (result: Vec<u8>){
    let mut result: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < x1.len()
    {
        let b1 = x1[i];
        let b2 = x2[i];
        let r = b1 & b2;
        result.push(r);
        i = i + 1;
    }
    result
}
}
fn main() {}