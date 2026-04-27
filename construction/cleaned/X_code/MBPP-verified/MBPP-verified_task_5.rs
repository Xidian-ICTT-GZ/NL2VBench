use vstd::prelude::*;
verus! {
fn extract_rear_chars(s: &Vec<Vec<char>>) -> (result: Vec<char>){
    let mut rear_chars: Vec<char> = Vec::with_capacity(s.len());
    let mut index = 0;
    while index < s.len()
    {
        let seq = &s[index];
        rear_chars.push(seq[seq.len() - 1]);
        index += 1;
    }
    rear_chars
}
fn main() {}
} 