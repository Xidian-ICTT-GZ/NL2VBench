use vstd::prelude::*;
verus! {
fn replace_with_colon(str1: &Vec<char>) -> (result: Vec<char>){
    let mut result: Vec<char> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if ((str1[index] == ' ') || (str1[index] == ',') || (str1[index] == '.')) {
            result.push(':');
        } else {
            result.push(str1[index]);
        }
        index += 1;
    }
    result
}
fn main() {}
} 