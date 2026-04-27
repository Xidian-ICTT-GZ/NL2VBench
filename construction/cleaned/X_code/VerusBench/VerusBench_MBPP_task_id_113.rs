use vstd::prelude::*;

fn main() {}

verus! {

fn is_digit(c: u8) -> (res: bool){
    c >= 48 && c <= 57
}

fn is_integer(text: &[u8]) -> (result: bool){
    let mut index = 0;
    while index < text.len() {
        if (!is_digit(text[index])) {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!