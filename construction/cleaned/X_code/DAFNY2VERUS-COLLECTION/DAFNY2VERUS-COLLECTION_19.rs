use vstd::prelude::*;
verus! {
fn has_only_one_distinct_element(a: &Vec<i32>) -> (result: bool){
    if a.len() == 0 {
        return true;
    }
    let first_element = a[0];
    let mut result = true;
    let mut i = 1;
    while i < a.len()
    {
        if a[i] != first_element {
            result = false;
        }
        i += 1;
    }
    result
}
fn main() {}
}