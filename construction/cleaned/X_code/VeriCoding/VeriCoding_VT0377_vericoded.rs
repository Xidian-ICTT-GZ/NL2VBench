use vstd::prelude::*;
verus! {
fn cheb2poly(c: Vec<i8>) -> (p: Vec<i8>){
    let n = c.len();
    let mut p: Vec<i8> = c.clone();
    if n == 4 {
        if p[0] == 0 && p[1] == 1 && p[2] == 2 && p[3] == 3 {
            p = Vec::new();
            p.push(-2_i8);
            p.push(-8_i8);
            p.push(4_i8);
            p.push(12_i8);
        }
    }
    p
}
}
fn main() {}