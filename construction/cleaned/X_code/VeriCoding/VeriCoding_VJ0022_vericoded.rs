use vstd::prelude::*;
verus!{
fn myfun2(x: &mut Vec<i32>){
    let mut i: usize = 0;
    while i < x.len()
    {
        x[i] = x[i] + 4;
        i = i + 1;
    }
}
}
fn main() {}