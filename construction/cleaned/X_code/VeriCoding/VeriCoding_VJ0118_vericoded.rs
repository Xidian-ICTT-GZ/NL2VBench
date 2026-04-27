use vstd::prelude::*;
verus! {
fn is_space_comma_dot(c: char) -> (result: bool){
    (c == ' ') || (c == ',') || (c == '.')
}
fn replace_with_colon(str1: &Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < str1.len()
    {
        let c = str1[i];
        if is_space_comma_dot(c) {
            result.push(':');
        } else {
            result.push(c);
        }
        i = i + 1;
    }
    result
}
}
fn main() {}