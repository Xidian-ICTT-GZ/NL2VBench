use vstd::prelude::*;
verus! {
fn find_first_even_index(lst: &Vec<i32>) -> (result: usize){
    let mut i: usize = 0;
    while i < lst.len()
    {
        if lst[i] % 2 == 0 {
            return i;
        }
        i = i + 1;
    }
    unreached()
}
fn find_first_odd_index(lst: &Vec<i32>) -> (result: usize){
    let mut i: usize = 0;
    while i < lst.len()
    {
        if lst[i] % 2 != 0 {
            return i;
        }
        i = i + 1;
    }
    unreached()
}
fn first_even_odd_indices(lst: Vec<i32>) -> (result: (usize, usize)){
    let even_idx = find_first_even_index(&lst);
    let odd_idx = find_first_odd_index(&lst);
    (even_idx, odd_idx)
}
}
fn main() {}