use vstd::prelude::*;

fn main() {}

verus! {

fn to_uppercase(str1: &[u8]) -> (result: Vec<u8>){
    let mut upper_case: Vec<u8> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if (str1[index] >= 97 && str1[index] <= 122) {
            upper_case.push((str1[index] - 32) as u8);
        } else {
            upper_case.push(str1[index]);
        }
        index += 1;
    }
    upper_case
}

} // verus!