use vstd::prelude::*;
verus! {
fn capitalize_first_letter(word: Vec<char>) -> (result: Vec<char>){
    let n = word.len();
    let mut result: Vec<char> = Vec::new();
    while result.len() < n
    {
        let i = result.len();
        if i == 0 {
            let c0 = word[0];
            let ch = if 'A' <= c0 && c0 <= 'Z' { c0 } else { 'A' };
            result.push(ch);
        } else {
            let tmp = word[i];
            result.push(tmp);
        }
    }
    result
}
}
fn main() {}