use vstd::prelude::*;
verus! {
fn lt_i8(a: i8, b: i8) -> bool { a < b }
fn less(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let mut res: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let ai: i8 = a[i];
        let bi: i8 = b[i];
        res.push(ai < bi);
        i += 1;
    }
    res
}
}
fn main() {}