use vstd::prelude::*;
verus! {
fn get_first_elements(lst: Vec<Vec<int>>) -> (result: Vec<int>){
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < lst.len()
    {
        result.push(lst[i][0]);
        i += 1;
    }
    result
}
fn main() {}
}