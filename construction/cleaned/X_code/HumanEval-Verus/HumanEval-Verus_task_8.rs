use vstd::prelude::*;
verus! {
fn below_zero(operation: &[i64]) -> (r: bool){
    let mut s = 0i128;
    for i in 0usize..operation.len()
    {
        s = s + operation[i] as i128;
        if s < 0 {
            return true;
        }
    }
    false
}
} 