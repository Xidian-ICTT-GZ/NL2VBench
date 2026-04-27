use vstd::prelude::*;
verus! {
fn min_second_value_first(s: &Vec<Vec<i32>>) -> (first_of_min_second: i32){
    let mut min_idx: usize = 0;
    let mut i: usize = 1;
    while i < s.len()
    {
        if s[i][1] < s[min_idx][1] {
            min_idx = i;
        }
        i = i + 1;
    }
    s[min_idx][0]
}
}
fn main() {}