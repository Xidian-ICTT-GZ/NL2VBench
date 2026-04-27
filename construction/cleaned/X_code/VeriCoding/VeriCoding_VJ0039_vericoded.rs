use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    if sum.len() == 0 {
        sum.push(4 * N);
    } else {
        sum[0] = 4 * N;
    }
}
}
fn main() {}