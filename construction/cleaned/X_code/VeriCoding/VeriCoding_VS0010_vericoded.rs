use vstd::prelude::*;
verus! {
fn clip(a: Vec<i8>, min: i8, max: i8) -> (result: Vec<i8>){
    let len = a.len();
    let mut result: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < len
    {
        let x = a[i];
        if x < min {
            result.push(min);
        } else if x > max {
            result.push(max);
        } else {
            result.push(x);
        }
        i += 1;
    }
    result
}
}
fn main() {}