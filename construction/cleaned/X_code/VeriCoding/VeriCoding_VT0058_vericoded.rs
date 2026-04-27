use vstd::prelude::*;
verus! {
fn numpy_insert<T>(arr: Vec<T>, idx: usize, value: T) -> (result: Vec<T>){
    let mut v = arr;
    v.insert(idx, value);
    v
}
}
fn main() {}