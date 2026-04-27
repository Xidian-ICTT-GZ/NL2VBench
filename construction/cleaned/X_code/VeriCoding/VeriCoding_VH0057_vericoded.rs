use vstd::prelude::*;
verus! {
fn same_chars(s0: Vec<char>, s1: Vec<char>) -> (result: bool){
    let mut i: usize = 0;
    while i < s0.len()
    {
        let c = s0[i];
        let mut found = false;
        let mut j: usize = 0;
        while j < s1.len()
        {
            if s1[j] == c {
                found = true;
            }
            j = j + 1;
        }
        if !found {
            return false;
        }
        i = i + 1;
    }
    let mut i: usize = 0;
    while i < s1.len()
    {
        let c = s1[i];
        let mut found = false;
        let mut j: usize = 0;
        while j < s0.len()
        {
            if s0[j] == c {
                found = true;
            }
            j = j + 1;
        }
        if !found {
            return false;
        }
        i = i + 1;
    }
    true
}
}
fn main() {}