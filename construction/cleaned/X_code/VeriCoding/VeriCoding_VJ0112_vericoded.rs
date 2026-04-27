use vstd::prelude::*;
verus! {
fn element_wise_module(arr1: &Vec<u32>, arr2: &Vec<u32>) -> (result: Vec<u32>){
    let n: usize = arr1.len();
    let mut res: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let a: u32 = arr1[i];
        let b: u32 = arr2[i];
        let m: u32 = a % b;
        res.push(m);
        i += 1;
    }
    res
}
}
fn main() {}