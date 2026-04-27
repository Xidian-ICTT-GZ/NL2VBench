use vstd::prelude::*;
verus! {
fn array_split(a: Vec<i32>) -> (ret: (Vec<i32>, Vec<i32>)){
    let n = a.len();
    let mut left = a;
    let k_usize: usize = if n == 0 { 0usize } else { 1usize };
    let mut right = left.split_off(k_usize);
    if n > 1 {
    }
    (left, right)
}
fn main() {
}
}