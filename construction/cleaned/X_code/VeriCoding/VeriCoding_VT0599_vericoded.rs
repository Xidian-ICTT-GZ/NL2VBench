use vstd::prelude::*;
verus! {
fn quantile(a: Vec<i8>, q: i8) -> (result: i8){
    let n = a.len();
    let mut i: usize = 1;
    let mut min_i: usize = 0;
    let mut max_i: usize = 0;
    let mut min_v: i8 = a[0];
    let mut max_v: i8 = a[0];
    while i < n
    {
        let v = a[i];
        if v < min_v {
            min_v = v;
            min_i = i;
        } else if v > max_v {
            max_v = v;
            max_i = i;
        }
        i += 1;
    }
    if q == 100 {
        max_v
    } else {
        min_v
    }
}
}
fn main() {}