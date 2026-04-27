use vstd::prelude::*;

fn main() {}

verus! {

fn replace_with_colon(str1: &[u8]) -> (result: Vec<u8>){
    let mut result: Vec<u8> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if ((str1[index] == 32) || (str1[index] == 44) || (str1[index] == 46)) {
            result.push(58);
        } else {
            result.push(str1[index]);
        }
        index += 1;
    }
    result
}

} // verus!