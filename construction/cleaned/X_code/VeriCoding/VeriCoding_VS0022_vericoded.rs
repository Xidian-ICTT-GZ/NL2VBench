use vstd::prelude::*;
verus! {
fn greater(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let n = a.len();
    let mut result: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let ai = a[i] as i32;
        let bi = b[i] as i32;
        let c = ai > bi;
        result.push(c);
        i = i + 1;
    }
    result
}
}
fn main() {}