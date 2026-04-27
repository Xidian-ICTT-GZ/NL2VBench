use vstd::prelude::*;
verus! {
fn replace_chars(str1: &Vec<char>, old_char: char, new_char: char) -> (result: Vec<char>){
    let mut result_str = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if str1[index] == old_char {
            result_str.push(new_char);
        } else {
            result_str.push(str1[index]);
        }
        index += 1;
    }
    result_str
}
fn main() {}
} 