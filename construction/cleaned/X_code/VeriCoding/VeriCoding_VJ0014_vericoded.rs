use vstd::prelude::*;
verus!{
fn myfun(a: &mut Vec<i32>, N: u32){
    let mut i: u32 = 0;
    while i < N
    {
        a.set(i as usize, if N % 2 == 0 { 0 } else { 1 });
        i += 1;
    }
}
}
fn main() {}