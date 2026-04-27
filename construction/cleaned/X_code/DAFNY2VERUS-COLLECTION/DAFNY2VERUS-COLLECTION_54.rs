use vstd::prelude::*;
use vstd::seq::*;
verus! {

fn max(a: Vec<u64>) -> (m: i64){
    if a.len() == 0 {
        return -1;
    }
    let mut i: usize = 0;
    let mut m: i64 = a[0] as i64;
    while i < a.len()
    {
        if (a[i] as i64) >= m {
            m = a[i] as i64;
        }
        i += 1;
    }
    m
}
fn checker() {
    let a = vec![1, 2, 3, 50, 5, 51];
    let n = max(a);
}
fn main() {}
}