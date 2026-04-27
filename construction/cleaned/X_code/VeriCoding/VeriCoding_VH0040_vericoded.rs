use vstd::prelude::*;
verus! {
fn decode_cyclic(s: &Vec<i8>) -> (res: Vec<i8>){
    let n = s.len();
    let mut res = s.clone();
    let mut i: usize = 0;
    let len_prefix = n - n % 3;
    while i < len_prefix
    {
        res.set(i, s[i + 2]);
        res.set(i + 1, s[i]);
        i = i + 3;
    }
    res
}
}
fn main() {}