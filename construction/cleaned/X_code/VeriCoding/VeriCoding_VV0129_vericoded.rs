use vstd::prelude::*;
verus! {
fn find_first_odd(a: &Vec<i32>) -> (result: Option<usize>){
    let n = a.len();
    let mut i: usize = 0;
    while i < n
    {
        if a[i] % 2 != 0 {
            return Some(i);
        } else {
            i = i + 1;
        }
    }
    None
}
}
fn main() {}