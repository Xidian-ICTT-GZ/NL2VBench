use vstd::prelude::*;
verus! {
fn has_only_one_distinct_element(arr: &Vec<i32>) -> (result: bool){
    let n = arr.len();
    let mut res: bool = true;
    let mut i: usize = 0;
    while i < n
    {
        let a0 = arr[0];
        let ai = arr[i];
        if a0 != ai {
            res = false;
        }
        i = i + 1;
    }
    res
}
}
fn main() {}