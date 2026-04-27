use vstd::prelude::*;
verus! {
fn to_lower_exec(c: char) -> (res: char){
    if 'A' <= c && c <= 'Z' {
        ((c as u8 - 'A' as u8 + 'a' as u8) as char)
    } else {
        c
    }
}
fn is_palindrome(text: Vec<char>) -> (result: bool){
    let n = text.len();
    let mut i: usize = 0;
    while i < n / 2
    {
        if to_lower_exec(text[i]) != to_lower_exec(text[n - 1 - i]) {
            return false;
        }
        i = i + 1;
    }
    return true;
}
}
fn main() {}