use vstd::prelude::*;
verus!{
fn binary_search(v: &Vec<u64>, k: u64) -> (result:usize){
    let mut low: usize = 0;
    let mut high: usize = v.len();
    while low < high
    {
        let mid: usize = low + (high - low) / 2;
        if v[mid] < k {
            low = mid + 1;
        } else if v[mid] > k {
            high = mid;
        } else {
            return mid;
        }
    }
    low
}
}
fn main() {}