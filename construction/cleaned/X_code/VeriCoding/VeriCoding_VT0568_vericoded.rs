use vstd::prelude::*;
verus! {
fn nanargmin(a: Vec<i8>) -> (result: usize){
    let mut min_index: usize = 0;
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] < a[min_index] {
            min_index = i;
        }
        i = i + 1;
    }
    min_index
}
}
fn main() {}