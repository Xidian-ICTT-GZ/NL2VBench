use vstd::prelude::*;
verus! {
fn min(a: &Vec<i32>, n: usize) -> (min: i32){
    let mut min = a[0];
    let mut i: usize = 1;
    while i < n && i < a.len()
    {
        if a[i] < min {
            min = a[i];
        }
        i = i + 1;
    }
    min
}
fn main() {}
}