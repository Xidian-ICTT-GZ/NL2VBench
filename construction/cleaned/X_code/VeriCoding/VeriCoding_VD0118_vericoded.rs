use vstd::prelude::*;
verus! {
fn mall_equal1(v: &[i32]) -> (b: bool){
    if v.len() == 0 {
        return true;
    }
    if v.len() == 1 {
        return true;
    }
    let first = v[0];
    let mut i = 1;
    while i < v.len()
    {
        if v[i] != first {
            return false;
        }
        i += 1;
    }
    true
}
fn main() {
}
}