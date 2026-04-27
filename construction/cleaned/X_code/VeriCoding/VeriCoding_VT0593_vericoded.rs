use vstd::prelude::*;
verus! {
fn nanmin(a: Vec<i8>) -> (result: i8){
    let mut min_val = a[0];
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] < min_val {
            min_val = a[i];
        }
        i = i + 1;
    }
    min_val
}
}
fn main() {}