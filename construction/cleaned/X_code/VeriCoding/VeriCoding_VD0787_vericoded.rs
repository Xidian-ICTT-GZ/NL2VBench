use vstd::prelude::*;
verus! {
fn is_min_heap(a: &Vec<i32>) -> (result: bool){
    let mut i: usize = 0;
    let n = a.len();
    while i < n / 2
    {
        let left_idx = 2 * i + 1;
        if left_idx < n {
            if a[i] > a[left_idx] {
                return false;
            }
        }
        let right_idx = 2 * i + 2;
        if right_idx < n {
            if a[i] > a[right_idx] {
                return false;
            }
        }
        i = i + 1;
    }
    true
}
}
fn main() {}