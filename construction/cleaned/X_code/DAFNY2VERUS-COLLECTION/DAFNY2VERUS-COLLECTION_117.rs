use vstd::prelude::*;
verus! {
fn max(a: &Vec<int>, n: usize) -> (max: int){
    let mut i: usize = 1;
    let mut max = a[0];
    while i < n
    {
        if a[i] > max {
            max = a[i];
        }
        i = i + 1;
    }
    max
}
fn main() {}
}