use vstd::prelude::*;
verus! {
fn split_and_append(list: &Vec<i32>, n: usize) -> (new_list: Vec<i32>){
    let mut left = list.clone();
    let mut right = left.split_off(n);
    right.append(&mut left);
    right
}
}
fn main() {}