use vstd::prelude::*;
verus! {
fn solve(s: Vec<char>) -> (result: &'static str){
    let mut stack: Vec<char> = Vec::new();
    let mut moves: u64 = 0;
    let mut i: usize = 0;
    while i < s.len()
    {
        let c = s[i];
        if stack.len() > 0 && stack[stack.len() - 1] == c {
            stack.pop();
            moves = moves + 1;
        } else {
            stack.push(c);
        }
        i = i + 1;
    }
    if moves % 2 == 1 {
        "Yes"
    } else {
        "No"
    }
}
}
fn main() {}