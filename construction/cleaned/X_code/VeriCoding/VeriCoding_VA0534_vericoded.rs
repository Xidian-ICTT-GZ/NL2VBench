use vstd::prelude::*;
verus! {
fn exec_turns_to_defeat(health: i8, strength: i8) -> (turns: i64){
    let h = health as i64;
    let s = strength as i64;
    (h + s - 1) / s
}
fn solve(a: i8, b: i8, c: i8, d: i8) -> (result: String){
    let takahashi_turns = exec_turns_to_defeat(c, b);
    let aoki_turns = exec_turns_to_defeat(a, d);
    if aoki_turns >= takahashi_turns {
        String::from_str("Yes")
    } else {
        String::from_str("No")
    }
}
}
fn main() {}