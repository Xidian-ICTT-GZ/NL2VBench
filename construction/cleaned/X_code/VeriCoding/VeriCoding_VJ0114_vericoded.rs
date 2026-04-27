use vstd::prelude::*;
verus! {
fn to_upper_char(c: char) -> (result: char){
    if c >= 'a' && c <= 'z' {
        ((c as u8) - 32) as char
    } else {
        c
    }
}
fn to_uppercase(str1: &Vec<char>) -> (result: Vec<char>){
    let n = str1.len();
    let mut res: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let c = str1[i];
        let u = to_upper_char(c);
        res.push(u);
        i = i + 1;
    }
    res
}
}
fn main() {}