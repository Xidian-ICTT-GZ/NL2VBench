use vstd::prelude::*;
verus! {
fn is_decimal_with_two_precision(s: &[u8]) -> (result: bool){
    let mut result = false;
    let mut i: usize = 0;
    while i < s.len()
    {
        if s[i] == '.' as u8 && s.len() - i - 1 == 2 {
            result = true;
        }
        i += 1;
    }
    result
}
fn main() {}
}