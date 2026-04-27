use vstd::prelude::*;
verus! {
fn linear_search(a: &Vec<i32>, e: i32) -> (result: usize){
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] == e {
            return i;
        }
        i = i + 1;
    }
    i
}
}
fn main() {}