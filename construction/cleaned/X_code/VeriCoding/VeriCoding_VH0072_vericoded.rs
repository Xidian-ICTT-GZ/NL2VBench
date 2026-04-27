use vstd::prelude::*;
verus! {
fn strange_sort_list_helper(s: Vec<i8>) -> (res: (Vec<i8>, Vec<i8>)){
    let res0 = s;
    let n = res0.len();
    let mut res1: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        if i % 2 == 0 {
            let k = i / 2;
            let v = res0[k];
            res1.push(v);
        } else {
            let q = (i - 1) / 2;
            let k = n - q - 1;
            let v = res0[k];
            res1.push(v);
        }
        i += 1;
    }
    (res0, res1)
}
}
fn main() {}