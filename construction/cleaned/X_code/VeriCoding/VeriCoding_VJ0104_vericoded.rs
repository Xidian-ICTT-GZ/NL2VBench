use vstd::prelude::*;
verus! {
fn check_match_at(main: &Vec<i32>, sub: &Vec<i32>, start: usize) -> (result: bool){
    let mut i = 0;
    while i < sub.len()
    {
        if main[start + i] != sub[i] {
            return false;
        }
        i += 1;
    }
    true
}
fn is_sub_array(main: &Vec<i32>, sub: &Vec<i32>) -> (result: bool){
    if sub.len() == 0 {
        return true;
    }
    if sub.len() > main.len() {
        return false;
    }
    let mut i = 0;
    while i <= main.len() - sub.len()
    {
        if check_match_at(main, sub, i) {
            return true;
        }
        i += 1;
    }
    false
}
}
fn main() {}