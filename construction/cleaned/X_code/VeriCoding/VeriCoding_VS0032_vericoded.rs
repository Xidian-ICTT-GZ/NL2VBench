use vstd::prelude::*;
verus! {
fn le_bool(a: i8, b: i8) -> (r: bool){
    a <= b
}
fn less_equal(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let n = a.len();
    let mut res: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let ai = a[i];
        let bi = b[i];
        let v = le_bool(ai, bi);
        let old_len = res.len();
        res.push(v);
        i = i + 1;
    }
    res
}
}
fn main() {}