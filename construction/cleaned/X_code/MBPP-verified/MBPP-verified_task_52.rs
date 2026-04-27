use vstd::prelude::*;
verus! {
fn contains(str: &Vec<char>, key: char) -> (result: bool){
    let mut i = 0;
    while i < str.len()
    {
        if (str[i] == key) {
            return true;
        }
        i += 1;
    }
    false
}
fn remove_chars(str1: &Vec<char>, str2: &Vec<char>) -> (result: Vec<char>){
    let mut output_str = Vec::new();
    let mut index: usize = 0;
    while index < str1.len()
    {
        if (!contains(str2, str1[index])) {
            output_str.push(str1[index]);
        }
        index += 1;
    }
    output_str
}
fn main() {}
} 