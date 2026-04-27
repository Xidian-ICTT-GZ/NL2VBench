use vstd::prelude::*;
verus! {
fn is_vowel_exec(c: char) -> (r: bool){
    let r = c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' ||
            c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U';
    r
}
fn get_vowel_replacement_exec(c: char) -> (r: char){
    let r = if c == 'a' { 'c' }
        else if c == 'e' { 'g' }
        else if c == 'i' { 'k' }
        else if c == 'o' { 'q' }
        else if c == 'u' { 'w' }
        else if c == 'A' { 'C' }
        else if c == 'E' { 'G' }
        else if c == 'I' { 'K' }
        else if c == 'O' { 'Q' }
        else if c == 'U' { 'W' }
        else { c };
    r
}
fn swap_case_exec(c: char) -> (r: char){
    let r = if 'a' <= c && c <= 'z' {
        ((c as u8 - 'a' as u8 + 'A' as u8) as char)
    } else if 'A' <= c && c <= 'Z' {
        ((c as u8 - 'A' as u8 + 'a' as u8) as char)
    } else {
        c
    };
    r
}
fn encode_char(c: char) -> (r: char){
    if c == ' ' {
        ' '
    } else {
        let v = is_vowel_exec(c);
        if v {
            let repl = get_vowel_replacement_exec(c);
            swap_case_exec(repl)
        } else {
            swap_case_exec(c)
        }
    }
}
fn encode(message: Vec<char>) -> (result: Vec<char>){
    let mut result: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < message.len()
    {
        let c = message[i];
        let r = encode_char(c);
        result.push(r);
        i += 1;
    }
    result
}
}
fn main() {}