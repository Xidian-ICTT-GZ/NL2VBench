use vstd::prelude::*;
verus! {
fn zap_negatives(a: &mut Vec<i32>){
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] < 0 {
            a[i] = 0;
        } else {
        }
        i += 1;
    }
}
fn main() {
}
}