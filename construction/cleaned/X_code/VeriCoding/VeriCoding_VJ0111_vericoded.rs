use vstd::prelude::*;
verus! {

fn remove_kth_element(list: &Vec<i32>, k: usize) -> (new_list: Vec<i32>){
    let mut new_list = list.clone();
    let idx: usize = k - 1;
    new_list.remove(idx);
    new_list
}
}
fn main() {}