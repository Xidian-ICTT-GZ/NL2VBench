use vstd::prelude::*;

fn main() {}

verus! {

fn replace_chars(str1: &[u8], old_char: u8, new_char: u8) -> (result: Vec<u8>){
    let mut result_str = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if str1[index] == old_char {
            result_str.push(new_char);
        } else {
            result_str.push(str1[index]);
        }
        index += 1;
    }
    result_str
}

} // verus!