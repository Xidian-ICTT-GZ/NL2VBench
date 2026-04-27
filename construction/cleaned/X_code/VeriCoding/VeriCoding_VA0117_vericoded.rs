use vstd::prelude::*;
verus! {
fn determine_winner(a: i8, b: i8, c: i8, d: i8) -> (result: String){
    let a_i32 = a as i32;
    let b_i32 = b as i32;
    let c_i32 = c as i32;
    let d_i32 = d as i32;
    let min_m = 3 * a_i32 / 10;
    let time_adj_m = a_i32 - a_i32 * c_i32 / 250;
    let misha_score_i32 = if min_m >= time_adj_m { min_m } else { time_adj_m };
    let min_v = 3 * b_i32 / 10;
    let time_adj_v = b_i32 - b_i32 * d_i32 / 250;
    let vasya_score_i32 = if min_v >= time_adj_v { min_v } else { time_adj_v };
    let res = if misha_score_i32 > vasya_score_i32 {
        "Misha".to_string()
    } else if vasya_score_i32 > misha_score_i32 {
        "Vasya".to_string()
    } else {
        "Tie".to_string()
    };
    res
}
}
fn main() {}