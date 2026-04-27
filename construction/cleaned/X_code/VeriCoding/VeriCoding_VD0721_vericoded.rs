use vstd::prelude::*;
verus! {
fn is_greater(n: i32, a: &[i32]) -> (result: bool){
    for i in 0..a.len()
    {
        if n <= a[i] {
            return false;
        }
    }
    true
}
fn main() {
}
}