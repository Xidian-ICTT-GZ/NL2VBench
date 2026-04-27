use vstd::prelude::*;
verus! {
fn encode_cyclic(s: Vec<i8>) -> (res: Vec<i8>){
    let mut res = s.clone();
    let n = s.len();
    let limit = n - n % 3;
    let mut i: usize = 0;
    while i < limit
    {
        let s_at_i = s[i];
        let s_at_i_plus_1 = s[i + 1];
        let s_at_i_plus_2 = s[i + 2];
        res.set(i, s_at_i_plus_1);
        res.set(i + 1, s_at_i_plus_2);
        res.set(i + 2, s_at_i);
        i = i + 3;
    }
    res
}
}
fn main() {}