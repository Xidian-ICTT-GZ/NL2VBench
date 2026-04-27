use vstd::prelude::*;
verus! {
fn copy<T: Copy>(a: &Vec<T>) -> (result: Vec<T>){
    let len = a.len();
    let mut result: Vec<T> = Vec::new();
    let mut i: usize = 0;
    while i < len
    {
        let v = a[i];
        result.push(v);
        i = i + 1;
    }
    result
}
}
fn main() {}