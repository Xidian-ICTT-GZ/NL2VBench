use vstd::prelude::*;
verus! {
fn reverse(a: &Vec<i32>) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut idx = 0;
    while idx < a.len()
    {
        result.push(a[a.len() - 1 - idx]);
        idx += 1;
    }
    result
}
}
fn main() {}