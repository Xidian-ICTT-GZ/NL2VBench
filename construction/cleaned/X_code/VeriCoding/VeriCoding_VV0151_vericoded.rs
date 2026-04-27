use vstd::prelude::*;
verus! {
fn longest_common_prefix(str1: &Vec<char>, str2: &Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < str1.len() && i < str2.len() && str1[i] == str2[i]
    {
        result.push(str1[i]);
        i += 1;
    }
    result
}
}
fn main() {}