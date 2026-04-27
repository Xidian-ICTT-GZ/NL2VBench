use vstd::prelude::*;
verus! {
fn is_lowercase(c: char) -> (result: bool){
    'a' <= c && c <= 'z'
}
fn is_uppercase(c: char) -> (result: bool){
    'A' <= c && c <= 'Z'
}
fn is_digit(c: char) -> (result: bool){
    '0' <= c && c <= '9'
}
fn check_contains_lowercase(v: &Vec<char>) -> (result: bool){
    let mut i = 0;
    while i < v.len()
    {
        if is_lowercase(v[i]) {
            return true;
        }
        i += 1;
    }
    false
}
fn check_contains_uppercase(v: &Vec<char>) -> (result: bool){
    let mut i = 0;
    while i < v.len()
    {
        if is_uppercase(v[i]) {
            return true;
        }
        i += 1;
    }
    false
}
fn check_contains_digit(v: &Vec<char>) -> (result: bool){
    let mut i = 0;
    while i < v.len()
    {
        if is_digit(v[i]) {
            return true;
        }
        i += 1;
    }
    false
}
fn trim_newline_impl(input: &Vec<char>) -> (result: Vec<char>){
    if input.len() > 0 && input[input.len() - 1] == '\n' {
        let mut result = Vec::new();
        let mut i = 0;
        while i < input.len() - 1
        {
            result.push(input[i]);
            i += 1;
        }
        result
    } else {
        input.clone()
    }
}
fn strip_whitespace_impl(input: &Vec<char>) -> (result: Vec<char>){
    if input.len() == 0 {
        input.clone()
    } else if input[0] == ' ' || input[0] == '\t' || input[0] == '\n' || input[0] == '\r' {
        let mut sub = Vec::new();
        let mut i = 1;
        while i < input.len()
        {
            sub.push(input[i]);
            i += 1;
        }
        strip_whitespace_impl(&sub)
    } else if input[input.len() - 1] == ' ' || input[input.len() - 1] == '\t' || input[input.len() - 1] == '\n' || input[input.len() - 1] == '\r' {
        let mut sub = Vec::new();
        let mut i = 0;
        while i < input.len() - 1
        {
            sub.push(input[i]);
            i += 1;
        }
        strip_whitespace_impl(&sub)
    } else {
        input.clone()
    }
}
fn solve(input: Vec<char>) -> (output: Vec<char>){
    let trimmed = trim_newline_impl(&input);
    let stripped = strip_whitespace_impl(&trimmed);
    let has_lowercase = check_contains_lowercase(&stripped);
    let has_uppercase = check_contains_uppercase(&stripped);
    let has_digit = check_contains_digit(&stripped);
    if stripped.len() >= 5 && has_lowercase && has_uppercase && has_digit {
        vec!['C', 'o', 'r', 'r', 'e', 'c', 't', '\n']
    } else {
        vec!['T', 'o', 'o', ' ', 'w', 'e', 'a', 'k', '\n']
    }
}
}
fn main() {}