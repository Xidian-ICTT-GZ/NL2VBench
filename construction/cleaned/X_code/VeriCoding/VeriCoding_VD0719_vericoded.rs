use vstd::prelude::*;
verus! {
fn slice_contains(s: &[i32], val: i32) -> (result: bool){
    let mut i: usize = 0;
    while i < s.len()
    {
        if s[i] == val {
            return true;
        }
        i = i + 1;
    }
    return false;
}
fn has_common_element(a: &[i32], b: &[i32]) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len()
    {
        if slice_contains(b, a[i]) {
            return true;
        }
        i = i + 1;
    }
    return false;
}
}
fn main() {}