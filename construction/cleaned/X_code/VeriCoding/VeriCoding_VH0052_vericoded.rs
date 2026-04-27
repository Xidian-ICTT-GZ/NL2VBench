use vstd::prelude::*;
verus! {
fn decode_shift(s: Vec<char>) -> (t: Vec<char>){
    let mut t = Vec::new();
    let mut i = 0;
    while i < s.len()
    {
        let c = s[i];
        let c_val = c as u8;
        let a_val = 'a' as u8;
        let decoded_val = ((c_val - a_val + 21) % 26 + a_val) as char;
        t.push(decoded_val);
        i += 1;
    }
    t
}
}
fn main() {}