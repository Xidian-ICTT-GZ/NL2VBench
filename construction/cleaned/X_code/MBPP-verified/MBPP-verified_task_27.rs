use vstd::prelude::*;
verus! {
fn to_toggle_case(str1: &Vec<char>) -> (toggle_case: Vec<char>){
    let mut toggle_case = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len()
    {
        if (str1[index] >= 'a' && str1[index] <= 'z') {
            toggle_case.push(((str1[index] as u8) - 32) as char);
        } else if (str1[index] >= 'A' && str1[index] <= 'Z') {
            toggle_case.push(((str1[index] as u8) + 32) as char);
        } else {
            toggle_case.push(str1[index]);
        }
        index += 1;
    }
    toggle_case
}
fn main() {}
} 