use vstd::prelude::*;

fn main() {}

verus! {

fn extract_rear_chars(s: &Vec<Vec<u8>>) -> (result: Vec<u8>){
    let mut rear_chars: Vec<u8> = Vec::with_capacity(s.len());
    let mut index = 0;
    while index < s.len() {
        let seq = &s[index];
        rear_chars.push(seq[seq.len() - 1]);
        index += 1;
    }
    rear_chars
}

} // verus!