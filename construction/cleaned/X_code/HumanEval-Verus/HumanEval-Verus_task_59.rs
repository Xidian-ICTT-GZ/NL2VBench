use vstd::prelude::*;
verus! {
fn is_palindrome(text: &str) -> (result: bool){
    let text_len: usize = text.unicode_len();
    for pos in 0..text_len / 2
    {
        if text.get_char(pos) != text.get_char(text_len - 1 - pos) {
            return false;
        }
    }
    true
}
} 