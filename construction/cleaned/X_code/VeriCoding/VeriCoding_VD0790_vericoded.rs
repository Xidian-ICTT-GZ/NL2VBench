use vstd::prelude::*;
verus! {
fn has_only_one_distinct_element(a: &[i32]) -> (result: bool){
    if a.len() <= 1 {
        return true;
    }
    let first_element = a[0];
    for i in 1..a.len()
    {
        if a[i] != first_element {
            return false;
        }
    }
    true
}
fn main() {}
}