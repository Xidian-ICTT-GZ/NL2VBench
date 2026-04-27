use vstd::prelude::*;
verus! {
fn equal(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let n = a.len();
    let mut res: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let v = a[i] == b[i];
        let old_i = i;
        let old_len = res.len();
        res.push(v);
        i = i + 1;
    }
    res
}
}
fn main() {}