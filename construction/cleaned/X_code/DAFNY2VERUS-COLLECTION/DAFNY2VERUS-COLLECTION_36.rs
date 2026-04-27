use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn minimum(a: Vec<int>) -> (m: int){
    let mut n: usize = 0;
    let mut m: int = a[0];
    while n != a.len()
    {
        if a[n] < m {
            m = a[n];
        }
        n = n + 1;
    }
    m
}
} 
fn main() {}