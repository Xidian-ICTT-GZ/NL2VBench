use vstd::prelude::*;
verus! {
fn maxArray(a: &[int]) -> (m: int){
    let mut m = a[0];
    let mut i = 1;
    while i < a.len()
    {
        if a[i] > m {
            m = a[i];
        }
        i = i + 1;
    }
    m
}
fn main() {
}
}