use vstd::prelude::*;
verus! {
fn is_digit_exec(c: char) -> (res: bool){
    let v: u32 = c as u32;
    v >= 48 && v <= 57
}
fn is_integer(text: &Vec<char>) -> (result: bool){
    let mut k: usize = 0;
    let mut res: bool = true;
    while k < text.len()
    {
        let c = text[k];
        let d = is_digit_exec(c);
        res = res && d;
        k = k + 1;
    }
    res
}
}
fn main() {}