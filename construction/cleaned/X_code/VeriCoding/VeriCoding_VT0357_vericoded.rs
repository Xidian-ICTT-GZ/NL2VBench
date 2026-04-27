use vstd::prelude::*;
verus! {
fn sign(x: Vec<i8>) -> (result: Vec<i8>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < x.len()
    {
        let val = x[i];
        let sign_val = if val < 0 {
            -1
        } else if val == 0 {
            0
        } else {
            1
        };
        result.push(sign_val);
        i += 1;
    }
    result
}
}
fn main() {}