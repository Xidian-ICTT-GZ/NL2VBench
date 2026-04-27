use vstd::prelude::*;
verus! {
fn longest(strings: Vec<Vec<char>>) -> (result: Option<Vec<char>>){
    if strings.len() == 0 {
        return Option::None;
    }
    let mut i: usize = 1;
    let mut best: usize = 0;
    while i < strings.len()
    {
        let curr_len = strings[i].len();
        let best_len = strings[best].len();
        if curr_len > best_len {
            best = i;
        }
        i = i + 1;
    }
    let value = strings[best].clone();

    Option::Some(value)
}
}
fn main() {}