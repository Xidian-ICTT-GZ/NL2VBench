use vstd::prelude::*;
verus!{
fn four_times(n: i32) -> (res: i32){
    let two = n + n;
    let four = two + two;
    four
}
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let val = four_times(N);
    sum.set(0usize, val);
}
}
fn main() {}