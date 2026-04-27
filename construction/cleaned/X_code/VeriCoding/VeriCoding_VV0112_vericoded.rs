use vstd::prelude::*;
verus! {
fn swap_first_and_last(a: &Vec<i32>) -> (result: Vec<i32>){
    let n = a.len();
    let mut res: Vec<i32> = Vec::new();
    if n == 1 {
        res.push(a[0]);
        res
    } else {
        res.push(a[n - 1]);
        let mut j: usize = 1;
        while j <= n - 2
        {
            res.push(a[j]);
            j += 1;
        }
        res.push(a[0]);
        res
    }
}
}
fn main() {}