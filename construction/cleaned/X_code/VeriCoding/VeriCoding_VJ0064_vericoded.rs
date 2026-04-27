use vstd::prelude::*;
verus! {
fn usize_min(a: usize, b: usize) -> (m: usize){
    if a <= b { a } else { b }
}
fn smallest_list_length(list: &Vec<Vec<i32>>) -> (min: usize){
    let n = list.len();
    let mut idx: usize = 0;
    let mut min_len: usize = list[0].len();
    while idx + 1 < n
    {
        let next = idx + 1;
        let cur = list[next].len();
        if cur < min_len {
            min_len = cur;
        }
        idx = next;
    }
    min_len
}
}
fn main() {}