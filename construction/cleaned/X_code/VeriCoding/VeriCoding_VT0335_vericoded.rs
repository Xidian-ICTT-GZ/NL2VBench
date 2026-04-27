use vstd::prelude::*;
verus! {
fn max_i8(a: i8, b: i8) -> (r: i8){
    if a >= b { a } else { b }
}
fn maximum(x1: Vec<i8>, x2: Vec<i8>) -> (result: Vec<i8>){
    let n = x1.len();
    let mut r: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let a = x1[i];
        let b = x2[i];
        let m = max_i8(a, b);
        r.push(m);
        i = i + 1;
    }
    r
}
}
fn main() {}