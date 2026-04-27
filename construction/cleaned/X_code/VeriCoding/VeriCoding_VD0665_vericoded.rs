use vstd::prelude::*;
verus! {
fn count_true(a: &[bool]) -> (result: usize){
    let mut count = 0usize;
    let mut i = 0usize;
    while i < a.len()
    {
        if a[i] {
            count = count + 1;
        }
        i = i + 1;
    }
    count
}
fn main() {
}
}