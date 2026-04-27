use vstd::prelude::*;
verus! {
exec fn is_in_group1(c: char) -> (result: bool){
    match c {
        'A' | 'E' | 'F' | 'H' | 'I' | 'K' | 'L' | 'M' | 'N' | 'T' | 'V' | 'W' | 'X' | 'Y' | 'Z' => true,
        _ => false,
    }
}
exec fn is_in_group2(c: char) -> (result: bool){
    match c {
        'B' | 'C' | 'D' | 'G' | 'J' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'U' => true,
        _ => false,
    }
}
exec fn check_all_in_group1(word: &Vec<char>) -> (result: bool){
    let mut i = 0;
    while i < word.len()
    {
        if !is_in_group1(word[i]) {
            return false;
        }
        i += 1;
    }
    true
}
exec fn check_all_in_group2(word: &Vec<char>) -> (result: bool){
    let mut i = 0;
    while i < word.len()
    {
        if !is_in_group2(word[i]) {
            return false;
        }
        i += 1;
    }
    true
}
fn solve(word: Vec<char>) -> (result: Vec<char>){
    if check_all_in_group1(&word) || check_all_in_group2(&word) {
        vec!['Y', 'E', 'S']
    } else {
        vec!['N', 'O']
    }
}
}
fn main() {}