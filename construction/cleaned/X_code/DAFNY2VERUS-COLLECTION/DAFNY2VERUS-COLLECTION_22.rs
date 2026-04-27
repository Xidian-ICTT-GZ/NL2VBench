use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn is_greater(n: i32, a: Vec<i32>) -> (result: bool){
    let mut result = true;
    let mut i: usize = 0;
    while i < a.len()
    {
        if n <= a[i] {
            result = false;
            return result;
        }
        i = i + 1;
    }
    result
}
fn main() {}
}