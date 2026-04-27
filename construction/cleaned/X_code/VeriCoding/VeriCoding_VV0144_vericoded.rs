use vstd::prelude::*;
verus! {
fn insert(oline: &Vec<char>, l: usize, nl: &Vec<char>, p: usize, at_pos: usize) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < at_pos
    {
        result.push(oline[i]);
        i += 1;
    }
    let mut j = 0;
    while j < p
    {
        result.push(nl[j]);
        j += 1;
    }
    let mut k = 0;
    while k < l - at_pos
    {
        result.push(oline[at_pos + k]);
        k += 1;
    }
    result
}
}
fn main() {}