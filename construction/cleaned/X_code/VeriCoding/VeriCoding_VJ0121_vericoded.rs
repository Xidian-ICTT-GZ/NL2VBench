use vstd::prelude::*;
verus! {
fn rotate_right(list: &Vec<u32>, n: usize) -> (new_list: Vec<u32>){
    let len = list.len();
    let s = len - (n % len);
    let mut v: Vec<u32> = Vec::new();
    let mut i = s;
    while i < len
    {
        v.push(list[i]);
        i += 1;
    }
    let mut j: usize = 0;
    while j < s
    {
        v.push(list[j]);
        j += 1;
    }
    v
}
}
fn main() {}