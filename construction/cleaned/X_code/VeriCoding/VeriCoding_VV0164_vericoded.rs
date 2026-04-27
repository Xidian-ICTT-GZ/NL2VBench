use vstd::prelude::*;
verus! {
fn replace(arr: &Vec<i32>, k: i32) -> (result: Vec<i32>){
    let mut res: Vec<i32> = Vec::new();
    while res.len() < arr.len()
    {
        let i = res.len();
        let a = arr[i];
        if a > k {
            res.push(-1);
        } else {
            res.push(a);
        }
    }
    res
}
}
fn main() {}