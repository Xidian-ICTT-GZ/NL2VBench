use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
    let mut total: i32 = 0;
    let mut i: usize = 0;
    while i < a.len()
    {
        a.set(i, 6);
        total = total + 6;
        i = i + 1;
    }
    sum.set(0, total);
}
}
fn main() {}