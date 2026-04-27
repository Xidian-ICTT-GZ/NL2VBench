use vstd::prelude::*;
verus! {
fn get_first_elements(lst: Vec<Vec<i32>>) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < lst.len()
    {
        let elem = lst[i][0];
        result.push(elem);
        i = i + 1;
    }
    result
}
}
fn main() {}