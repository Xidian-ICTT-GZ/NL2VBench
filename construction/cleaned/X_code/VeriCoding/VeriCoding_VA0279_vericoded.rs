use vstd::prelude::*;
verus! {
fn safe_sum(coins: &Vec<i8>) -> (result: i32){
    let sum = coins[0] as i32 + coins[1] as i32 + coins[2] as i32 + coins[3] as i32 + coins[4] as i32;
    sum
}
fn solve(coins: Vec<i8>) -> (result: i8){
    let total = safe_sum(&coins);
    if total > 0 && total % 5 == 0 {
        (total / 5) as i8
    } else {
        -1
    }
}
}
fn main() {}