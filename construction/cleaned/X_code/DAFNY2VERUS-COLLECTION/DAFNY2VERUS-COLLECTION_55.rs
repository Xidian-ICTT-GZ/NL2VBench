use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn find(a: Vec<i32>, key: i32) -> (index: usize){
    let mut index: usize = 0;
    while index < a.len() && a[index] != key
    {
        index = index + 1;
    }
    index
}
fn main() {}
}