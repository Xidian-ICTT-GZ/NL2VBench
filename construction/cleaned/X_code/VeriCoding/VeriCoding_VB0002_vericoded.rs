use vstd::prelude::*;
verus! {
fn compare(s1: Vec<char>, s2: Vec<char>) -> (res: i32){
    if s1.len() < s2.len() {
        -1
    } else if s1.len() > s2.len() {
        1
    } else {
        let mut i = 0;
        while i < s1.len()
        {
            if s1[i] < s2[i] {
                return -1;
            } else if s1[i] > s2[i] {
                return 1;
            }
            i += 1;
        }
        0
    }
}
}
fn main() {}