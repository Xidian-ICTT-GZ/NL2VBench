use vstd::prelude::*;
verus! {
fn factorial_of_last_digit(n: u64) -> (fact: u64){
    let d: u64 = n % 10;
    if d == 0 {
        1u64
    } else if d == 1 {
        1u64
    } else if d == 2 {
        2u64
    } else if d == 3 {
        6u64
    } else if d == 4 {
        24u64
    } else if d == 5 {
        120u64
    } else if d == 6 {
        720u64
    } else if d == 7 {
        5040u64
    } else if d == 8 {
        40320u64
    } else {
        362880u64
    }
}
fn main() {
}
}