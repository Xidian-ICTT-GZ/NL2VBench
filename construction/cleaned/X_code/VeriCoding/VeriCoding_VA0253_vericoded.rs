use vstd::prelude::*;
verus! {
fn solve(n: i8, s: i8, a: Vec<i8>) -> (result: i8){
    if n == 1 {
        if s == a[0] {
            return 1;
        } else {
            return 0;
        }
    }
    let mut all_greater = true;
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] <= s {
            all_greater = false;
        }
        i = i + 1;
    }
    if all_greater {
        return 0;
    }
    return 0;
}
}
fn main() {}