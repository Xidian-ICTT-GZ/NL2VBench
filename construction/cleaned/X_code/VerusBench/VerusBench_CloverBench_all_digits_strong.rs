use vstd::prelude::*;

fn main() {}
verus! {

fn is_ascii_digit(c: char) -> (r: bool){
    c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7'
        || c == '8' || c == '9'
}

fn all_digits(s: String) -> (result: bool){
    let mut result = true;
    let mut i = 0;
    while i < s.as_str().unicode_len() {
        if !is_ascii_digit(s.as_str().get_char(i)) {
            return false;
        }
        i += 1;
    }
    true
}

} // verus!