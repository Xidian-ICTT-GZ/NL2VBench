use vstd::prelude::*;
verus! {
fn all_characters_same(char_arr: &Vec<char>) -> (result: bool){
    if char_arr.len() <= 1 {
        return true;
    }
    let mut index = 1;
    while index < char_arr.len()
    {
        if char_arr[0] != char_arr[index] {
            return false;
        }
        index += 1;
    }
    true
}
fn main() {}
} 