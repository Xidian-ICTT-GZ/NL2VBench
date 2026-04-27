use vstd::prelude::*;
verus! {
fn max_array(a: &Vec<i32>) -> (result: i32){
    let mut i: usize = 1;
    let mut max: i32 = a[0];
    let mut idx: usize = 0;
    while i < a.len()
    {
        if a[i] > max {
            max = a[i];
            idx = i;
        }
        i += 1;
    }
    max
}
}
fn main() {}