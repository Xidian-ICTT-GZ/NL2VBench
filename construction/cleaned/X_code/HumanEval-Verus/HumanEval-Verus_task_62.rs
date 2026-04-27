use vstd::prelude::*;
verus! {
fn below_threshold(l: &[i32], t: i32) -> (result: bool){
    for i in 0..l.len()
    {
        if l[i] >= t {
            return false;
        }
    }
    true
}
} 