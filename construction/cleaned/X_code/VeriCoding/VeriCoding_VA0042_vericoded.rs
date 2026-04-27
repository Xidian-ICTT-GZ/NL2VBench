use vstd::prelude::*;
verus! {
fn check_disconnected(n: i8, m: i8, horizontal: &Vec<char>, vertical: &Vec<char>) -> (b: bool){
    let h0 = horizontal[0];
    let hn_1 = horizontal[(n - 1) as usize];
    let v0 = vertical[0];
    let vm_1 = vertical[(m - 1) as usize];
    (h0 == '>' && v0 == 'v') ||
    (h0 == '<' && vm_1 == 'v') ||
    (hn_1 == '>' && v0 == '^') ||
    (hn_1 == '<' && vm_1 == '^')
}
fn solve(n: i8, m: i8, horizontal: Vec<char>, vertical: Vec<char>) -> (result: Vec<char>){
    if check_disconnected(n, m, &horizontal, &vertical) {
        vec!['N', 'O', '\n']
    } else {
        vec!['Y', 'E', 'S', '\n']
    }
}
}
fn main() {}