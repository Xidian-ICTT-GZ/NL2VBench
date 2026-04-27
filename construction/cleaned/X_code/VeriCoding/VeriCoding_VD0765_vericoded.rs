use vstd::prelude::*;
verus! {
fn remove_element(s: &Vec<i32>, k: usize) -> (v: Vec<i32>){
    let mut res: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < k
    {
        res.push(s[i]);
        i += 1;
    }
    i += 1;
    while i < s.len()
    {
        res.push(s[i]);
        i += 1;
    }
    res
}
fn main() {}
}