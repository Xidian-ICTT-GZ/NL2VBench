use vstd::prelude::*;
verus! {
fn binary_search(a: &Vec<i32>, key: i32) -> (n: usize){
    let mut lo: usize = 0;
    let mut hi: usize = a.len();
    while lo < hi
    {
        let mid = lo + (hi - lo) / 2;
        if a[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo 
}
fn main() {}
}