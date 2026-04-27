use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: usize){
    let init_len = sum.len();
    while sum.len() > 0
    {
        let _ = sum.pop();
    }
    sum.push(0);
    let val: i32 = 4 * (N as i32);
    sum[0] = val;
}
}
fn main() {}