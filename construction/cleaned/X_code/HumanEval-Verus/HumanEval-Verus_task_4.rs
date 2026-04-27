use vstd::prelude::*;
verus! {
fn below_zero(operations: Vec<i32>) -> (result: bool){
    let mut s = 0i32;
    for k in 0..operations.len()
    {
        s = s + operations[k];
        if s < 0 {
            return true;
        }
    }
    false
}
} 