use vstd::prelude::*;

fn main() {}

verus! {

fn to_lowercase(str1: &[u8]) -> (result: Vec<u8>){
    let mut lower_case: Vec<u8> = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if (str1[index] >= 65 && str1[index] <= 90) {
            lower_case.push((str1[index] + 32) as u8);

        } else {
            lower_case.push(str1[index]);
        }
        index += 1;
    }
    lower_case
}

} // verus!