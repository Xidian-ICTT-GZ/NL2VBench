use vstd::prelude::*;
verus! {
fn reverse(a: &Vec<char>) -> (b: Vec<char>){
    let mut b = Vec::new();
    let mut j = 0;
    while j < a.len()
    {
        let idx = a.len() - 1 - j;
        b.push(a[idx]);
        j = j + 1;
    }
    b
}
fn main() {
}
}