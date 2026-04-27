use vstd::prelude::*;
verus! {
fn check_below_threshold(l: Vec<i8>, t: i8) -> (result: bool){
    let mut i = 0;
    while i < l.len()
    {
        if l[i] >= t {
            return false;
        }
        i += 1;
    }
    true
}
}
fn main() {}