use vstd::prelude::*;
verus! {
fn reverse(a: &Vec<char>) -> (b: Vec<char>){
    let mut b: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        b.push(a[a.len() - 1 - i]);
        i = i + 1;
    }
    b
}
fn main() {
}
}