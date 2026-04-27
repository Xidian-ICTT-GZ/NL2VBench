use vstd::prelude::*;
verus! {
fn nanmax(a: Vec<i8>) -> (result: i8){
    let mut max_val = a[0];
    let mut i = 1;
    while i < a.len()
    {
        if a[i] > max_val {
            max_val = a[i];
        }
        i += 1;
    }
    max_val
}
}
fn main() {}