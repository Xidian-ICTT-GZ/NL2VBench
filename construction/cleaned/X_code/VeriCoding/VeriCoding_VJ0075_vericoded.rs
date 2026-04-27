use vstd::prelude::*;
verus! {
fn insert_before_each(arr: &Vec<i32>, elem: i32) -> (result: Vec<i32>){
    let mut out: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < arr.len()
    {
        out.push(elem);
        out.push(arr[i]);
        i += 1;
    }
    out
}
}
fn main() {}