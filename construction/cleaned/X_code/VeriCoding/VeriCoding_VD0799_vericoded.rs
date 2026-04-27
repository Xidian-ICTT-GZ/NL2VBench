use vstd::prelude::*;
verus! {
fn is_even_at_index_even(lst: &Vec<i32>) -> (result: bool){
    let mut i: usize = 0;
    while i < lst.len()
    {
        if i % 2 == 0 {
            let val = lst[i];
            if val % 2 != 0 {
                return false;
            }
        }
        i = i + 1;
    }
    true
}
}
fn main() {}