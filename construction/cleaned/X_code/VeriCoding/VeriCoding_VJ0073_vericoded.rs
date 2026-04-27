use vstd::prelude::*;
verus! {
fn replace_last_element(first: &Vec<i32>, second: &Vec<i32>) -> (replaced_list: Vec<i32>){
    let n1 = first.len();
    let n2 = second.len();
    let mut res: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < n1 - 1
    {
        res.push(first[i]);
        i += 1;
    }
    let mut j: usize = 0;
    while j < n2
    {
        res.push(second[j]);
        j += 1;
    }
    res
}
}
fn main() {}