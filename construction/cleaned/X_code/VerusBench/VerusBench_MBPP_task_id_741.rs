use vstd::prelude::*;

fn main() {}

verus! {

fn all_characters_same(char_arr: &[u8]) -> (result: bool){
    if char_arr.len() <= 1 {
        return true;
    }
    let mut index = 1;
    while index < char_arr.len() {
        if char_arr[0] != char_arr[index] {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!