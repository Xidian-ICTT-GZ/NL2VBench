use vstd::prelude::*;
verus! {
fn is_happy(s: Vec<char>) -> (result: bool){
    if s.len() < 3 {
        return false;
    }
    let mut i = 0;
    while i <= s.len() - 3
    {
        if s[i] == s[i+1] || s[i] == s[i+2] || s[i+1] == s[i+2] {
            return false;
        }
        i += 1;
    }
    true
}
}
fn main() {}