use vstd::prelude::*;
verus! {
fn count_digits(text: &Vec<char>) -> (count: usize){
    let mut i: usize = 0;
    let mut cnt: usize = 0;
    while i < text.len()
    {
        let c = text[i];
        let is_d = ((c as u8) >= 48u8) && ((c as u8) <= 57u8);
        if is_d {
            cnt = cnt + 1;
        }
        i = i + 1;
    }
    cnt
}
}
fn main() {}