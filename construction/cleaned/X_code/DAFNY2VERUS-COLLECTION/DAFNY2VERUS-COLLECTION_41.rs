use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn find_smallest(s: Vec<int>) -> (min: int){
    let mut min = s[0];
    let mut i = 1;
    while i < s.len()
    {
        if s[i] < min {
            min = s[i];
        }
        i += 1;
    }
    min
}
fn main() {}
}