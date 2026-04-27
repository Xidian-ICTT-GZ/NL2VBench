use vstd::prelude::*;
verus! {

fn solve(input: Vec<char>) -> (result: Vec<char>){
    let mut out: Vec<char> = Vec::new();
    if input[0] == input[2] {
        out.push('H');
    } else {
        out.push('D');
    }
    out.push('\n');
    out
}
}
fn main() {}