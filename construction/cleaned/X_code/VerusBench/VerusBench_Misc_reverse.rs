use vstd::prelude::*;
fn main() {}

verus!{
fn reverse(v: &mut Vec<u64>){
    let length = v.len();
    let mut n: usize = 0;
    while n < length / 2
    {
        let x = v[n];
        let y = v[length - 1 - n];
        v.set(n, y);
        v.set(length - 1 - n, x);

        n = n + 1;
    }
}
}