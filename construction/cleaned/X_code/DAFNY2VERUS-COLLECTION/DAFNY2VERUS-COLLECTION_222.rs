use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn reverse(a: &mut Vec<i32>){
    let mut i: usize = 0;
    while i < a.len() / 2
    {
        let j = a.len() - 1 - i;

        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
        i = i + 1;
    }
}
fn main() {}
}