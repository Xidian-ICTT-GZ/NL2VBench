use vstd::prelude::*;
verus! {
fn exec_flip_char(c: char) -> (r: char){
    if 'a' <= c && c <= 'z' {
        let r0: char = (((c as u8) - ('a' as u8)) + ('A' as u8)) as char;
        r0
    } else if 'A' <= c && c <= 'Z' {
        let r0: char = (((c as u8) - ('A' as u8)) + ('a' as u8)) as char;
        r0
    } else {
        c
    }
}
fn flip_case(s: Vec<char>) -> (result: Vec<char>){
    let n: usize = s.len();
    let mut result: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let c = s[i];
        let r = exec_flip_char(c);
        result.push(r);
        i = i + 1;
    }
    result
}
}
fn main() {}