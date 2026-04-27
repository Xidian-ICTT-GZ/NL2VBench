use vstd::prelude::*;
verus! {
fn find_min(a: &mut [i32], from: usize, to: usize) -> (index: usize){
    let mut min_index = from;
    let mut i = from + 1;
    while i < to
    {
        if a[i] < a[min_index] {
            min_index = i;
        }
        i += 1;
    }
    min_index
}
fn main() {
}
}