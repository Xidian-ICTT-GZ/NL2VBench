use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: bool){
    if n < 12 {
        if n == 1 || n == 7 || n == 9 || n == 10 || n == 11 {
            false
        } else {
            true
        }
    } else if 12 < n && n < 30 {
        false
    } else if 69 < n && n < 80 {
        false
    } else if 89 < n {
        false
    } else {
        let last_digit = n % 10;
        if last_digit != 1 && last_digit != 7 && last_digit != 9 {
            true
        } else {
            false
        }
    }
}
}
fn main() {}