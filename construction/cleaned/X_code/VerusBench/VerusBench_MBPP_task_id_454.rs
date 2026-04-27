use vstd::prelude::*;

fn main() {}

verus! {

fn contains_z(text: &[u8]) -> (result: bool){
    let mut index = 0;
    while index < text.len() {
        if text[index] == 90 || text[index] == 122 {
            return true;
        }
        index += 1;
    }
    false
}

} // verus!