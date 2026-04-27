use vstd::prelude::*;
verus! {
fn choose_yes_no(cond: bool) -> (result: &'static str){
    if cond {
        "YES"
    } else {
        "NO"
    }
}
fn solve(a: i8, b: i8, c: i8) -> (result: &'static str){
    let cond = (a == 5i8 && b == 5i8 && c == 7i8)
        || (a == 5i8 && b == 7i8 && c == 5i8)
        || (a == 7i8 && b == 5i8 && c == 5i8);
    let res = choose_yes_no(cond);
    res
}
}
fn main() {}