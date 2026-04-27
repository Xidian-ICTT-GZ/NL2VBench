use vstd::prelude::*;
verus! {
fn replace_blanks_with_chars(str1: &Vec<char>, ch: char) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < str1.len()
    {
        if str1[i] == ' ' {
            result.push(ch);
        } else {
            result.push(str1[i]);
        }
        i += 1;
    }
    result
}
}
fn main() {}