use vstd::prelude::*;
verus! {
fn concat(a: &[i32], b: &[i32]) -> (c: Vec<i32>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < a.len()
    {
        result.push(a[i]);
        i += 1;
    }
    let mut j = 0;
    while j < b.len()
    {
        result.push(b[j]);
        j += 1;
    }
    result
}
fn main() {
}
}