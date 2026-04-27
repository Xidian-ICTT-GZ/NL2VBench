use vstd::prelude::*;
verus! {
fn contains_consecutive_numbers(a: &[i32]) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len() - 1
    {
        if let Some(val) = a[i].checked_add(1) {
            if val == a[i+1] {
                return true;
            }
        }
        i = i + 1;
    }
    return false;
}
}
fn main() {}