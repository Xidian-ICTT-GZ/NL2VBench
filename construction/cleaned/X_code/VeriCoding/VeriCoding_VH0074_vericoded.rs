use vstd::prelude::*;
verus! {
fn smallest_change(arr: Vec<i8>) -> (changes: usize){
    let n = arr.len();
    let mut i: usize = 0;
    let mut changes: usize = 0;
    while i < n / 2
    {
        if arr[i] != arr[n - 1 - i] {
            changes = changes + 1;
        }
        i = i + 1;
    }
    changes
}
}
fn main() {}