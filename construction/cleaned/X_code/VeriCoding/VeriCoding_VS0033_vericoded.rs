use vstd::prelude::*;
verus! {
fn max(a: Vec<i8>) -> (result: i8){
    let mut max_val = a[0];
    let mut idx = 1;
    while idx < a.len()
    {
        if a[idx] > max_val {
            max_val = a[idx];
        }
        idx += 1;
    }
    max_val
}
}
fn main() {}