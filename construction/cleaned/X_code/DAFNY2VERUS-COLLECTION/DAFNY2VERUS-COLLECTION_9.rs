use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn linear_search(a: &Vec<i32>, e: i32) -> (n: usize){
    let mut n = 0;
    while n != a.len()
    {
        if e == a[n] {
            return n; 
        }
        n = n + 1;
    }
    n 
}
fn main() {}
}