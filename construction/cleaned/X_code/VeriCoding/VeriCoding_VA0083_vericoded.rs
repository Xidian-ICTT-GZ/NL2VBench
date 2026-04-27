use vstd::prelude::*;
verus! {

fn solve(n: i8) -> (result: i8){
    let hundreds = n / 100;
    let remainder_after_hundreds = n % 100;
    let twenties = remainder_after_hundreds / 20;
    let remainder_after_twenties = remainder_after_hundreds % 20;
    let tens = remainder_after_twenties / 10;
    let remainder_after_tens = remainder_after_twenties % 10;
    let fives = remainder_after_tens / 5;
    let ones = remainder_after_tens % 5;
    let total = hundreds + twenties + tens + fives + ones;
    total
}
}
fn main() {}