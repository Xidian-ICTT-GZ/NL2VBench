use vstd::prelude::*;
verus! {
fn fibonacci(n: usize) -> (ret: Vec<i32>){
    let mut v = Vec::new();
    v.push(0i32);
    v.push(1i32);
    let mut i: usize = 2;
    while i < n
    {
        let prev1 = v[i - 1];
        let prev2 = v[i - 2];
        let next = prev2 + prev1;
        v.push(next);
        i = i + 1;
    }
    v
}
}
fn main() {}