use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, b: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let mut i: usize = 0;
    let n_val = (N + 1) as i32;
    while i < N as usize
    {
        a[i] = n_val;
        i = i + 1;
    }
}
}
fn main() {}