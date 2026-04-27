use vstd::prelude::*;
verus! {
fn myfun1(x: &Vec<i32>) -> (max_index: usize){
    let n = x.len();
    let mut max_index: usize = 0;
    let mut i: usize = 1;
    while i < n
    {
        if x[i] > x[max_index] {
            max_index = i;
        } else {
        }
        i = i + 1;
    }
    max_index
}
}
fn main() {}