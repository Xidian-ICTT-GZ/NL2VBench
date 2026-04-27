use vstd::prelude::*;
verus! {
fn zeros_like(a: &Vec<i32>) -> (result: Vec<i32>){
    let mut res: Vec<i32> = Vec::new();
    while res.len() < a.len()
    {
        res.push(0);
    }
    res
}
}
fn main() {}