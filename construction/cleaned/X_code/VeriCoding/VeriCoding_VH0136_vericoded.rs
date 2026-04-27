use vstd::prelude::*;
verus! {
fn check_if_last_char_is_a_letter(txt: &str) -> (result: bool){
    let len = txt.unicode_len();
    if len == 0 {
        return false;
    }
    let last_char = txt.get_char(len - 1);
    let is_letter = (last_char >= 'a' && last_char <= 'z') || (last_char >= 'A' && last_char <= 'Z');
    if !is_letter {
        return false;
    }
    if len == 1 {
        return true;
    }
    let second_last = txt.get_char(len - 2);
    second_last == ' '
}
}
fn main() {}