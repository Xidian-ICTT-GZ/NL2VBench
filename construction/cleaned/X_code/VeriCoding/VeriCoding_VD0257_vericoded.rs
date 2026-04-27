use vstd::prelude::*;
verus! {
fn swap3(a: &mut Vec<i32>, h: usize, i: usize, j: usize){
    let old_seq = a.clone();
    let tmp: i32 = a[h];
    a[h] = a[i];
    a[i] = a[j];
    a[j] = tmp;
}
fn main() {}
}