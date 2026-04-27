use vstd::prelude::*;
verus! {
fn replace_blanks_with_chars(str1: &Vec<char>, ch: char) -> (result: Vec<char>){
    let mut out_str: Vec<char> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if (str1[index] == ' ') {
            out_str.push(ch);
        } else {
            out_str.push(str1[index]);
        }
        index += 1;
    }
    out_str
}
fn main() {}
} 