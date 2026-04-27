use vstd::prelude::*;
verus! {
fn replace(arr: &mut Vec<i32>, k: i32){
    let n = arr.len();
    let mut i: usize = 0;
    while i < n
    {
        let x = arr[i];
        if x > k {
            arr.set(i, -1);
        }
        i += 1;
    }
}
fn main() {}
}