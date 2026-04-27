use vstd::prelude::*;
verus! {
fn all_elements_equal(a: &[i32], n: i32) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] != n {
            return false;
        }
        i += 1;
    }
    true
}
fn main() {
}
}