use vstd::prelude::*;
verus! {
fn filter_vowels_array(xs: &[char]) -> (ys: Vec<char>){
    let mut result: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < xs.len()
    {
        let c = xs[i];
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            result.push(c);
        } else {
        }
        i = i + 1;
    }
    result
}
fn main() {}
}