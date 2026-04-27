use vstd::prelude::*;
verus! {
fn solve_cake_problem(a: i8, b: i8) -> (result: &'static str){
    if a <= 8 && b <= 8 {
        "Yay!"
    } else {
        ":("
    }
}
}
fn main() {}