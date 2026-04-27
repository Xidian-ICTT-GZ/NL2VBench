use vstd::prelude::*;
verus! {
fn min_array(a: &[i32]) -> (r: i32){
    let mut min_val: i32 = a[0];
    let mut j: usize = 1usize;
    while j < a.len()
    {
        let old = min_val;
        let ai: i32 = a[j];
        if ai < old {
            min_val = ai;
        } else {
        }
        j = j + 1usize;
    }
    min_val
}
fn main() {}
}