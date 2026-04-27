use vstd::prelude::*;
verus! {
fn days_in_month_exec(m: i8) -> (res: i8){
    let r: i8;
    if m == 2 {
        r = 28;
    } else if m == 4 || m == 6 || m == 9 || m == 11 {
        r = 30;
    } else {
        r = 31;
    }
    r
}
fn solve(m: i8, d: i8) -> (result: i8){
    let dim: i8 = days_in_month_exec(m);
    let n: i8 = d - 1 + dim - 1;
    let res: i8;
    if n <= 27 {
        res = 4; 
    } else if n <= 34 {
        res = 5; 
    } else {
        res = 6; 
    }
    res
}
}
fn main() {}