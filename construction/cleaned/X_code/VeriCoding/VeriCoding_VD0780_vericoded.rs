use vstd::prelude::*;
verus! {
fn reverse_upto_k(s: &mut Vec<i32>, k: usize){
    let mut i: usize = 0;
    while i < k / 2
    {
        let j = k - 1 - i;
        let temp_i = s[i];
        let temp_j = s[j];
        s.set(i, temp_j);
        s.set(j, temp_i);
        i = i + 1;
    }
}
}
fn main() {}