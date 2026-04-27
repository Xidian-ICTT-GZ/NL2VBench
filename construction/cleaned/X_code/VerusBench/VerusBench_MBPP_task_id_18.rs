use vstd::prelude::*;

fn main() {}

verus! {

fn contains(str: &[u8], key: u8) -> (result: bool){
    let mut i = 0;
    while i < str.len() {
        if (str[i] == key) {
            return true;
        }
        i += 1;
    }
    false
}

fn remove_chars(str1: &[u8], str2: &[u8]) -> (result: Vec<u8>){
    let mut output_str = Vec::new();
    let mut index: usize = 0;

    while index < str1.len() {
        if (!contains(str2, str1[index])) {
            output_str.push(str1[index]);
        }
        index += 1;
    }
    output_str
}

} // verus!