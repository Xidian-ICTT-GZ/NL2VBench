use vstd::prelude::*;
verus! {
fn reverse(a: &[i32]) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut i = a.len();
    while i > 0
    {
        i = i - 1;
        result.push(a[i]);
    }
    result
}
}
fn main() {}