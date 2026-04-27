use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    sum.clear();
    let two_n: i32 = N + N;
    sum.push(two_n);
}
}
fn main() {}