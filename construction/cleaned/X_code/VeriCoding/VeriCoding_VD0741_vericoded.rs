use vstd::prelude::*;
verus! {
fn is_lower_case_exec(c: char) -> (result: bool){
    97 <= c as u32 && c as u32 <= 122
}
fn is_upper_case_exec(c: char) -> (result: bool){
    65 <= c as u32 && c as u32 <= 90
}
fn to_upper_case_exec(c: char) -> (result: char){
    ((c as u32 - 32) as u8) as char
}
fn to_lower_case_exec(c: char) -> (result: char){
    ((c as u32 + 32) as u8) as char
}
fn toggle_case(s: Vec<char>) -> (v: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len()
    {
        let c = s[i];
        let new_char = if is_lower_case_exec(c) {
            to_upper_case_exec(c)
        } else if is_upper_case_exec(c) {
            to_lower_case_exec(c)
        } else {
            c
        };
        result.push(new_char);
        i = i + 1;
    }
    result
}
fn main() {
}
}