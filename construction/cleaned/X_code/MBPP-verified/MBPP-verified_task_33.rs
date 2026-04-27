use vstd::prelude::*;
verus! {
fn remove_kth_element(list: &Vec<i32>, k: usize) -> (new_list: Vec<i32>){
    let mut new_list = Vec::new();
    let mut index = 0;
    while index < (k - 1)
    {
        new_list.push(list[index]);
        index += 1;
    }
    let mut index = k;
    while index < list.len()
    {
        new_list.push(list[index]);
        index += 1;
    }
    new_list
}
fn main() {}
} 