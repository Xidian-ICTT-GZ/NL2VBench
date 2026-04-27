use vstd::prelude::*;
verus! {
fn contains_consecutive_numbers(a: &Vec<i32>) -> (result: bool){
    if a.len() < 2 {
        return false;
    }
    let mut i: usize = 0;
    while i < a.len() - 1
    {
        if a[i] < i32::MAX && a[i] + 1 == a[i + 1] {
            return true;
        }
        i = i + 1;
    }
    return false;
}
}
fn main() {}