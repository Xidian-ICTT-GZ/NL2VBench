use vstd::prelude::*;
verus! {
fn numpy_argmin(a: Vec<i8>) -> (result: usize){
    let n: usize = a.len();
    let mut res: usize = 0;
    let mut min_val: i8 = a[0];
    let mut i: usize = 1;
    while i < n
    {
        let curr: i8 = a[i];
        if curr < min_val {
            let old_i = i;
            res = i;
            min_val = curr;
        } else if curr == min_val {
        } else {
        }
        i = i + 1;
    }
    res
}
}
fn main() {}