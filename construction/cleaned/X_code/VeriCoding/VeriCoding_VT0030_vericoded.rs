use vstd::prelude::*;
verus! {
fn ones_like<T>(a: &Vec<T>) -> (result: Vec<i32>){
    let n: usize = a.len();
    let mut v: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        i = i + 1;
        v.push(1i32);
    }
    v
}
}
fn main() {}