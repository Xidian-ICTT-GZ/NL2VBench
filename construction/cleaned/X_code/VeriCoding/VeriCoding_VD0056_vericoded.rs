use vstd::prelude::*;
verus! {
fn copy(src: &[i32], s_start: usize, dest: &[i32], d_start: usize, len: usize) -> (r: Vec<i32>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < dest.len()
    {
        if i >= d_start && i < d_start + len 
        {
            let offset = i - d_start;
            let src_index = s_start + offset;
            result.push(src[src_index]);
        } else {
            result.push(dest[i]);
        }
        i = i + 1;
    }
    result
}
fn main() {
}
}