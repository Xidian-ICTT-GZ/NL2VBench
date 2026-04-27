use vstd::prelude::*;
verus! {
fn determine_food_safety(x: i8, a: i8, b: i8) -> (outcome: &'static str){
    let days_rt: i32 = (b as i32) - (a as i32);
    let res: &'static str;
    if days_rt <= 0 {
        res = "delicious";
    } else if days_rt <= x as i32 {
        res = "safe";
    } else {
        res = "dangerous";
    }
    res
}
}
fn main() {}