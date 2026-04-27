use vstd::prelude::*;
verus! {
fn solve(n: i8, m: i8, tasks: Vec<i8>) -> (result: i8){
    let len = tasks.len();
    let mut i: usize = 0;
    let mut last: i8 = 0;
    let mut seen: bool = false;
    while i < len
    {
        let x = tasks[i];
        i = i + 1;
        last = x;
        seen = true;
    }
    let res: i8 = last - 1;
    res
}
}
fn main() {}