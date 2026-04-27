use vstd::prelude::*;
verus! {

fn binary_search(a: &[i32], key: i32) -> (n: usize){
    let mut low: usize = 0;
    let mut high: usize = a.len();
    while low < high
    {
        let mid = low + (high - low) / 2;
        if a[mid] < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
fn main() {}
}