use vstd::prelude::*;
verus! {
fn list_deep_clone(arr: &Vec<u64>) -> (copied: Vec<u64>){
    let copied = arr.clone();
    copied
}
}
fn main() {}