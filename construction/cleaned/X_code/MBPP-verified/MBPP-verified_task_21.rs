use vstd::prelude::*;
verus! {
fn count_frequency(arr: &Vec<char>, key: char) -> (frequency: usize){
    let mut index = 0;
    let mut counter = 0;
    while index < arr.len()
    {
        if (arr[index] == key) {
            counter += 1;
        }
        index += 1;
    }
    counter
}
fn first_repeated_char(str1: &Vec<char>) -> (repeated_char: Option<(usize, char)>){
    let input_len = str1.len();
    let mut index = 0;
    while index < str1.len()
    {
        if count_frequency(&str1, str1[index]) > 1 {
            return Some((index, str1[index]));
        }
        index += 1;
    }
    None
}
fn main() {}
} 