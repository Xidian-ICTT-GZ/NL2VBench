use vstd::prelude::*;
verus! {
fn find_max_in_prefix(v: &[i32], end: usize) -> (res: (i32, usize)){
    let mut max_val = v[0];
    let mut max_idx = 0;
    let mut i: usize = 1;
    while i <= end
    {
        if v[i] > max_val {
            max_val = v[i];
            max_idx = i;
        }
        i = i + 1;
    }
    (max_val, max_idx)
}
fn find_min_in_suffix(v: &[i32], start: usize) -> (res: (i32, usize)){
    let mut min_val = v[start];
    let mut min_idx = start;
    let mut i: usize = start + 1;
    while i < v.len()
    {
        if v[i] < min_val {
            min_val = v[i];
            min_idx = i;
        }
        i = i + 1;
    }
    (min_val, min_idx)
}
fn barrier(v: &[i32], p: usize) -> (b: bool){
    if p + 1 >= v.len() {
        return true;
    }
    let (max_left, k_max) = find_max_in_prefix(v, p);
    let (min_right, l_min) = find_min_in_suffix(v, p + 1);
    let b = max_left < min_right;
    if b {
    } else {
    }
    b
}
}
fn main() {}