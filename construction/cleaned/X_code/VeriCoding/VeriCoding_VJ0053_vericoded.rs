use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, b: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let n = b.len();
    let val: i32 = N + 2;
    let mut i: usize = 0;
    while i < n
    {
        b[i] = val;
        i += 1;
    }
}
}
fn main() {}