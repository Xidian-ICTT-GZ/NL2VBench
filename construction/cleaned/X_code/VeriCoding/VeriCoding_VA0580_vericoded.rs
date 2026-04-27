use vstd::prelude::*;
verus! {
fn next_char_exec(c: char) -> (d: char){
    let d = ((c as u8) + 1) as char;
    d
}
fn solve(input: Vec<char>) -> (output: Vec<char>){
    let c = input[0];
    let d = next_char_exec(c);
    let mut out: Vec<char> = Vec::new();
    out.push(d);
    out.push('\n');
    out
}
}
fn main() {}