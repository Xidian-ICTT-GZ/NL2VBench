use vstd::prelude::*;
verus! {
fn all_characters_same(s: &[u8]) -> (result: bool){
    if s.len() <= 1 {
        return true;
    }
    let first_char = s[0];
    let mut result = true;
    let mut i: usize = 1;
    while i < s.len()
    {
        if s[i] != first_char {
            result = false;
        }
        i += 1;
    }
    result
}
fn main() {}
}