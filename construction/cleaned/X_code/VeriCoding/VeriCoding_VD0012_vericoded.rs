use vstd::prelude::*;
verus! {
fn array_to_sequence(arr: &[bool; 10]) -> (res: Vec<bool>){
    let mut v: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < 10usize
    {
        let val = arr[i];
        v.push(val);
        i = i + 1;
    }
    v
}
fn main() {
}
}