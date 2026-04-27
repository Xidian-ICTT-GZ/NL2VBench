use vstd::prelude::*;
verus! {
fn is_odd_exec(n: i32) -> (b: bool){
    n % 2 != 0
}
fn is_odd_at_index_odd(a: &Vec<i32>) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len()
    {
        if i % 2 == 1 {
            if !is_odd_exec(a[i]) {
                return false;
            }
        }
        i = i + 1;
    }
    true
}
}
fn main() {}