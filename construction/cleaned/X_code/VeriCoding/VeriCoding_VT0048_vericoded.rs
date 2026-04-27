use vstd::prelude::*;
verus! {
fn copyto<T: Copy>(dst: Vec<T>, src: Vec<T>, mask: Vec<bool>) -> (result: Vec<T>){
    let mut result = dst;
    let mut i = 0;
    while i < result.len()
    {
        if mask[i] {
            result.set(i, src[i]);
        }
        i += 1;
    }
    result
}
}
fn main() {}