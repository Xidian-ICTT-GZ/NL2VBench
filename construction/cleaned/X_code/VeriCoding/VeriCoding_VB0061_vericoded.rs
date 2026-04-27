use vstd::prelude::*;
verus! {
fn zeros(n: usize) -> (s: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < n
    {
        result.push('0');
        i += 1;
    }
    result
}
}
fn main() {}