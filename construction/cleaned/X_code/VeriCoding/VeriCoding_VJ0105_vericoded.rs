use vstd::prelude::*;
verus! {
fn interleave(s1: &Vec<i32>, s2: &Vec<i32>, s3: &Vec<i32>) -> (res: Vec<i32>){
    let n = s1.len();
    let mut res: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        res.push(s1[i]);
        res.push(s2[i]);
        res.push(s3[i]);
        i = i + 1;
    }
    res
}
}
fn main() {}