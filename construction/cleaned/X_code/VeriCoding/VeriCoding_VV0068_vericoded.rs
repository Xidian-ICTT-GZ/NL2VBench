use vstd::prelude::*;
verus! {
fn search_insert(xs: &Vec<i32>, target: i32) -> (result: usize){
    let mut i: usize = 0;
    while i < xs.len() && xs[i] < target
    {
        i += 1;
    }
    if i < xs.len() {
    }
    i
}
}
fn main() {}