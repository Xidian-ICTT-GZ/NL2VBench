use vstd::prelude::*;
verus! {
fn check_all_lowercase(v: &Vec<char>) -> (r: bool){
    let mut i: usize = 0;
    let mut ok: bool = true;
    let mut bad: bool = false;
    while i < v.len()
    {
        let c = v[i];
        let is_lower = c >= 'a' && c <= 'z';
        if !is_lower {
            ok = false;
        }
        bad = bad || !is_lower;
        i += 1;
    }
    let r = if v.len() == 0 { false } else { ok };
    r
}
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let r = check_all_lowercase(&input);
    if r {
        vec!['a']
    } else {
        vec!['A']
    }
}
}
fn main() {}