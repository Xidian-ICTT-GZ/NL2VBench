use vstd::prelude::*;
verus! {
fn encode_shift(s: Vec<char>) -> (t: Vec<char>){
    let mut t = Vec::new();
    let mut i = 0;
    while i < s.len()
    {
        let c = s[i];
        let encoded = ((c as u8 - 'a' as u8 + 5) % 26 + 'a' as u8) as char;
        t.push(encoded);
        i += 1;
    }
    t
}
}
fn main() {}