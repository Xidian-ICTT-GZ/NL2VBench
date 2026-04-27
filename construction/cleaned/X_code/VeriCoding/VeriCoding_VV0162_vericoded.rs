use vstd::prelude::*;
verus! {
fn remove_front(a: &Vec<i32>) -> (result: Vec<i32>){
    let mut b = a.clone();
    let res = b.split_off(1);
    res
}
}
fn main() {}