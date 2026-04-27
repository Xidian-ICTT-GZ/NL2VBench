use vstd::prelude::*;

fn main() {}

verus! {

fn count_frequency(arr: &[u8], key: u8) -> (frequency: usize){
    let mut index = 0;
    let mut counter = 0;
    while index < arr.len() {
        if (arr[index] == key) {
            counter += 1;
        }
        index += 1;
    }
    counter
}

fn first_repeated_char(str1: &[u8]) -> (repeated_char: Option<(usize, u8)>){
    let mut index = 0;
    while index < str1.len() {
        if count_frequency(&str1, str1[index]) > 1 {
            return Some((index, str1[index]));
        }
        index += 1;
    }
    None
}

} // verus!