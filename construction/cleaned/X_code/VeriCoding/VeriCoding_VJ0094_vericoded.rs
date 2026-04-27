use vstd::prelude::*;
verus! {
fn is_upper_case_exec(c: char) -> (b: bool){
    let cu = c as u32;
    cu >= 65 && cu <= 90
}
fn count_uppercase_upto(text: &Vec<char>, n: usize) -> (count: usize){
    if n == 0 {
        0usize
    } else {
        let prev = count_uppercase_upto(text, n - 1);
        let c = text[n - 1];
        let inc: usize = if is_upper_case_exec(c) { 1 } else { 0 };
        let res = prev + inc;
        res
    }
}
fn count_uppercase(text: &Vec<char>) -> (count: u64){
    let n = text.len();
    let acc: usize = count_uppercase_upto(text, n);
    acc as u64
}
}
fn main() {}