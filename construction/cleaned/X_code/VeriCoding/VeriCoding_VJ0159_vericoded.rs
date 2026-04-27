use vstd::prelude::*;
verus! {
fn replace_chars(s: &[char], old: char, new: char) -> (result: Vec<char>){
    let mut v: Vec<char> = Vec::new();
    v.reserve(s.len());
    let mut i: usize = 0;
    while i < s.len()
    {
        let c = if s[i] == old { new } else { s[i] };
        v.push(c);
        i += 1;
    }
    v
}
}
fn main() {}