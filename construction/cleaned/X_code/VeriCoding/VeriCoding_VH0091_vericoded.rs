use vstd::prelude::*;
verus! {
fn encrypt(s: Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len()
    {
        let c = s[i];
        let encrypted = if 'a' <= c && c <= 'z' {
            let offset = ((c as u8 - 'a' as u8 + 4) % 26) + 'a' as u8;
            offset as char
        } else if 'A' <= c && c <= 'Z' {
            let offset = ((c as u8 - 'A' as u8 + 4) % 26) + 'A' as u8;
            offset as char
        } else {
            c
        };
        result.push(encrypted);
        i = i + 1;
    }
    result
}
}
fn main() {}