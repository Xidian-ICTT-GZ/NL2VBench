use vstd::prelude::*;
verus! {
fn to_lowercase(str1: &Vec<char>) -> (result: Vec<char>){
    let mut lower_case: Vec<char> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if (str1[index] >= 'A' && str1[index] <= 'Z') {
            lower_case.push(((str1[index] as u8) + 32) as char);
        } else {
            lower_case.push(str1[index]);
        }
        index += 1;
    }
    lower_case
}
fn main() {}
} 