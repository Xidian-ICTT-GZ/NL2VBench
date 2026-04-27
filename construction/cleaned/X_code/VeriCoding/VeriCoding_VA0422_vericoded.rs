use vstd::prelude::*;
verus! {
fn determine_winner(x: i8, y: i8) -> (winner: &'static str){
    let d_i8: i8 = x - y;
    let ad_i8: i8 = if d_i8 >= 0 { d_i8 } else { -d_i8 };
    let winner = if ad_i8 > 1 { "Alice" } else { "Brown" };
    winner
}
}
fn main() {}