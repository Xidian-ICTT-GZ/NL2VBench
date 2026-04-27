use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, b: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let mut i: usize = 0;
    let n_usize = N as usize;
    let value = 2 * N + 1;
    while i < n_usize
    {
        a.set(i, value);
        i += 1;
    }
}
}
fn main() {}