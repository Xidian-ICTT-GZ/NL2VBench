use vstd::prelude::*;
verus! {
fn lower_char_exec(c: char) -> (result: char){
    if 'A' <= c && c <= 'Z' {
        let r_u8: u8 = (c as u8) + 32u8;
        let r: char = r_u8 as char;
        r
    } else {
        c
    }
}
fn to_lowercase(s: &Vec<char>) -> (result: Vec<char>){
    let mut res: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < s.len()
    {
        let c: char = s[i];
        let lc: char = lower_char_exec(c);
        res.push(lc);
        i = i + 1;
    }
    res
}
}
fn main() {}