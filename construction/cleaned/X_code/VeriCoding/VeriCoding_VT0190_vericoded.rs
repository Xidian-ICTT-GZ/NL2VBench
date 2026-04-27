use vstd::prelude::*;
verus! {
fn r_(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<i8>){
    let mut r = a;
    let mut bb = b;
    r.append(&mut bb);
    r
}
}
fn main() {}