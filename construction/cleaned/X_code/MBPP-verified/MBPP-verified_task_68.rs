use vstd::prelude::*;
verus! {
fn is_digit(c: char) -> (res: bool){
    (c as u32) >= 48 && (c as u32) <= 57
}
fn is_integer(text: &Vec<char>) -> (result: bool){
    let mut index = 0;
    while index < text.len()
    {
        if (!is_digit(text[index])) {
            return false;
        }
        index += 1;
    }
    true
}
fn main() {}
} 