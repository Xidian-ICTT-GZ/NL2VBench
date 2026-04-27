use vstd::prelude::*;
verus!{
fn times_five(x: i32) -> (r: i32){
    5 * x
}
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let val = times_five(N);
    let idx: usize = 0;
    sum.set(idx, val);
}
}
fn main() {}