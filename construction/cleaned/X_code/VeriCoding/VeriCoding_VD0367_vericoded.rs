use vstd::prelude::*;
verus! {
fn yarra(arr: &Vec<char>) -> (outarr: Vec<char>){
    let len = arr.len();
    let mut outarr = Vec::new();
    let mut i: usize = 0;
    while i < len
    {
        outarr.push(arr[len - 1 - i]);
        i = i + 1;
    }
    outarr
}
}
fn main() {}