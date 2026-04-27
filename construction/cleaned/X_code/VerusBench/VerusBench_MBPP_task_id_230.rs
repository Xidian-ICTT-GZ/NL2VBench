use vstd::prelude::*;

fn main() {}

verus! {

fn replace_blanks_with_chars(str1: &[u8], ch: u8) -> (result: Vec<u8>){
    let mut out_str: Vec<u8> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if str1[index] == 32 {
            out_str.push(ch);
        } else {
            out_str.push(str1[index]);
        }
        index += 1;
    }
    out_str
}

} // verus!