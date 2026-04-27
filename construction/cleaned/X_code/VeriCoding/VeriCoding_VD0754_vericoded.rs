use vstd::prelude::*;
verus! {
fn array_to_seq(a: &[i32]) -> (s: Vec<i32>){
    let mut s: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let ai = a[i];
        s.push(ai);
        i += 1;
    }
    s
}
fn main() {
}
}