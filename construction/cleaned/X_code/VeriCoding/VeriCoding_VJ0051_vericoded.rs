use vstd::prelude::*;
verus!{
fn set_at(a: &mut Vec<i32>, i: usize, val: i32){
    a[i] = val;
}
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let val: i32 = N + 1;
    let mut i: usize = 0;
    while i < a.len()
    {
        set_at(a, i, val);
        i = i + 1;
    }
}
}
fn main() {}