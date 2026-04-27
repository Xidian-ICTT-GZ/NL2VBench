use vstd::prelude::*;
verus! {
fn frombuffer(buffer: &Vec<u8>, count: usize, offset: usize) -> (result: Vec<u8>){
    let mut result = Vec::with_capacity(count);
    let mut i: usize = 0;
    while i < count
    {
        result.push(buffer[offset + i]);
        i = i + 1;
    }
    result
}
}
fn main() {}