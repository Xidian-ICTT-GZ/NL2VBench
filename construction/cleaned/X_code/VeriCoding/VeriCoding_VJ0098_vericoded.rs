use vstd::prelude::*;
verus! {
fn is_upper_case_exec(c: char) -> (ret: bool){
    c >= 'A' && c <= 'Z'
}
fn shift32_exec(c: char) -> (ret: char){
    let u: u8 = c as u8;
    let shifted: u8 = u + 32;
    let r: char = shifted as char;
    r
}
fn to_lowercase(str1: &Vec<char>) -> (result: Vec<char>){
    let n = str1.len();
    let mut result_vec: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let c = str1[i];
        let is_up = is_upper_case_exec(c);
        let r = if is_up { shift32_exec(c) } else { c };
        result_vec.push(r);
        i = i + 1;
    }
    result_vec
}
}
fn main() {}