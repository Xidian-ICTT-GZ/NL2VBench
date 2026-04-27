use vstd::prelude::*;
verus! {
fn is_vowel_exec(c: char) -> (b: bool){
    let r = c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    r
}
fn build_vowel_vec() -> (v: Vec<char>){
    let mut out: Vec<char> = Vec::new();
    out.push('v');
    out.push('o');
    out.push('w');
    out.push('e');
    out.push('l');
    out
}
fn build_consonant_vec() -> (v: Vec<char>){
    let mut out: Vec<char> = Vec::new();
    out.push('c');
    out.push('o');
    out.push('n');
    out.push('s');
    out.push('o');
    out.push('n');
    out.push('a');
    out.push('n');
    out.push('t');
    out
}
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let c = input[0];
    let b = is_vowel_exec(c);
    if b {
        let v = build_vowel_vec();
        v
    } else {
        let v = build_consonant_vec();
        v
    }
}
}
fn main() {}