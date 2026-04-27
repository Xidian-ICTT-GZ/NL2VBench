use vstd::prelude::*;
verus! {
fn to_uppercase(str1: &Vec<char>) -> (result: Vec<char>){
    let mut upper_case: Vec<char> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if (str1[index] >= 'a' && str1[index] <= 'z') {
            upper_case.push(((str1[index] as u8) - 32) as char);
        } else {
            upper_case.push(str1[index]);
        }
        index += 1;
    }
    upper_case
}
fn main() {}
} 