use vstd::prelude::*;
verus! {
fn search_range_helper(a: &[i32], x: i32, lo: usize, hi: usize) -> (k: usize){
    if lo == hi {
        lo
    } else {
        let mid: usize = (lo + hi) / 2;
        if a[mid] < x {
            search_range_helper(a, x, mid + 1, hi)
        } else {
            search_range_helper(a, x, lo, mid)
        }
    }
}
fn search_1000(a: &[i32], x: i32) -> (k: usize){
    search_range_helper(a, x, 0usize, 1000usize)
}
fn main() {
}
}