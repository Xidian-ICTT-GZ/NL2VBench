use vstd::prelude::*;
verus! {
fn bin_search(a: &[int], k: int) -> (b: bool){
    if a.len() == 0 {
        return false;
    }
    let mut lo: usize = 0;
    let mut hi: usize = a.len();
    while lo < hi
    {
        let mid = lo + (hi - lo) / 2;
        if a[mid] == k {
            return true;
        } else if a[mid] < k {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    false
}
fn main() {
}
}