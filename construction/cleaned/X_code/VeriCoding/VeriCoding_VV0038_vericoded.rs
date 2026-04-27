use vstd::prelude::*;
verus! {
fn max_of_list(lst: &Vec<usize>) -> (result: usize){
    let n = lst.len();
    let mut idx: usize = 0;
    let mut i: usize = 1;
    while i < n
    {
        if lst[i] > lst[idx] {
            idx = i;
        }
        i += 1;
    }
    let result0 = lst[idx];
    result0
}
}
fn main() {}