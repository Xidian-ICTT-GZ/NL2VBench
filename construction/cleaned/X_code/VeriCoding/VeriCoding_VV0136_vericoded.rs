use vstd::prelude::*;
verus! {
fn copy(src: &Vec<i32>, s_start: usize, dest: &Vec<i32>, d_start: usize, len: usize) -> (result: Vec<i32>){
    let mut result = dest.clone();
    let mut i: usize = 0;
    while i < len
    {
        let val = src[s_start + i];
        result.set(d_start + i, val);
        i = i + 1;
    }
    result
}
}
fn main() {}