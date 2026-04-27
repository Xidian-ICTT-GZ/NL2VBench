use vstd::prelude::*;
verus! {
fn find_first_odd(a: &[i32]) -> (result: (bool, usize)){
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] % 2 != 0 {
            return (true, i);
        } else {
        }
        i += 1;
    }
    (false, 0)
}
fn main() {}
}