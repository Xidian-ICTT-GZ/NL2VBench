use vstd::prelude::*;
verus!{
fn set_first(sum: &mut Vec<i32>, val: i32){
    sum[0] = val;
}
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let two_n: i32 = 2 * N;
    set_first(sum, two_n);
}
}
fn main() {}