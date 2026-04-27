use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn cube_elements(a: Vec<int>) -> (cubed: Vec<int>){
    let mut cubed_array: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let val = a[i];
        cubed_array.push(val * val * val);
        i += 1;
    }
    cubed_array
}
fn main() {}
}