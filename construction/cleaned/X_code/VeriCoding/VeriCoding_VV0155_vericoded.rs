use vstd::prelude::*;
verus! {
fn min_array(a: &Vec<i32>) -> (result: i32){
    let mut min_val = a[0];
    let mut idx = 1;
    while idx < a.len()
    {
        if a[idx] < min_val {
            min_val = a[idx];
        }
        idx += 1;
    }
    min_val
}
}
fn main() {}