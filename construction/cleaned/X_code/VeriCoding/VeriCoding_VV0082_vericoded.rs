use vstd::prelude::*;
verus! {
fn find_smallest(s: &Vec<nat>) -> (result: Option<nat>){
    if s.len() == 0usize {
        return None;
    }
    let n: usize = s.len();
    let mut i: usize = 1usize;
    let mut min: nat = s[0usize];
    while i < n
    {
        let v: nat = s[i];
        if v < min {
            min = v;
        }
        i += 1usize;
    }
    Some(min)
}
}
fn main() {}