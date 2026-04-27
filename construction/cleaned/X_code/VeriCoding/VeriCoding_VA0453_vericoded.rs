use vstd::prelude::*;
verus! {
fn exec_is_power_of_two(n_in: i8) -> (res: bool){
    let mut n = n_in;
    while n > 1
    {
        if n % 2 != 0 {
            return false;
        }
        n = n / 2;
    }
    true
}
fn exec_is_limited_prime(p: i8) -> (res: bool){
    if p <= 1 {
        false
    } else if p == 2 {
        true
    } else if p % 2 == 0 {
        false
    } else {
        true
    }
}
fn exec_determine_winner(n_in: i8) -> (res: &'static str){
    if n_in == 1 {
        "FastestFinger"
    } else if n_in == 2 {
        "Ashishgup"
    } else if exec_is_power_of_two(n_in) {
        "FastestFinger"
    } else if n_in % 4 != 2 {
        "Ashishgup"
    } else {
        if exec_is_limited_prime(n_in / 2) {
            "FastestFinger"
        } else {
            "Ashishgup"
        }
    }
}
fn solve(input: Vec<i8>) -> (result: Vec<&'static str>){
    let mut result: Vec<&'static str> = Vec::new();
    let mut i: usize = 1;
    while i < input.len()
    {
        let n = input[i];
        let winner = exec_determine_winner(n);
        result.push(winner);
        i = i + 1;
    }
    result
}
}
fn main() {}