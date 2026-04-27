use vstd::prelude::*;
verus! {
fn sign_of(x: i32) -> (r: i32){
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else {
        -1
    }
}
fn sign(a: Vec<i32>) -> (result: Vec<i32>){
    let mut result: Vec<i32> = Vec::new();
    let mut k: usize = 0;
    while k < a.len()
    {
        let x = a[k];
        if x > 0 {
            let old_len: usize = result.len();
            result.push(1);
        } else if x == 0 {
            let old_len: usize = result.len();
            result.push(0);
        } else {
            let old_len: usize = result.len();
            result.push(-1);
        }
        k = k + 1;
    }
    result
}
}
fn main() {}