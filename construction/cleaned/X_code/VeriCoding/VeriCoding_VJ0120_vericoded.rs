use vstd::prelude::*;
verus! {
fn all_characters_same(char_arr: &Vec<char>) -> (result: bool){
    if char_arr.len() <= 1 {
        return true;
    }
    let mut i: usize = 1;
    while i < char_arr.len()
    {
        if char_arr[i] != char_arr[0] {
            return false;
        }
        i = i + 1;
    }
    true
}
}
fn main() {}