use vstd::prelude::*;
verus! {
fn replace_chars(str1: &Vec<char>, old_char: char, new_char: char) -> (result: Vec<char>){
    let n = str1.len();
    let mut out: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let ch = if str1[i] == old_char { new_char } else { str1[i] };
        out.push(ch);
        i = i + 1;
    }
    out
}
}
fn main() {}