use vstd::prelude::*;
verus! {
fn below_zero(operations: Vec<i32>) -> (result: bool){
    let mut s = 0i32;
    let mut num_overflows: usize = 0;
    for k in 0..operations.len()
    {
        let op = operations[k];
        if op >= 0 {
            if s > i32::MAX - op {
                s += op - i32::MAX - 1;
                num_overflows += 1;
            } else {
                s += op;
            }
        } else {
            s += op;
            if s < 0 {
                if num_overflows == 0 {
                    return true;
                }
                num_overflows -= 1;
                s = s + i32::MAX + 1;
            }
        }
    }
    false
}
} 